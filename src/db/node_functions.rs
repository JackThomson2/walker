use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::Result;
use tokio_postgres::Client;
use tokio_postgres::NoTls;
use tokio_postgres::Statement;

use crate::napi::fast_str::FastStr;
use crate::napi::postgres_rows::PostgresRows;
use crate::tokio_workers;

#[napi]
pub struct DbConnection {
    clients: Vec<Arc<Client>>,
    pos: AtomicUsize,
}

#[napi(ts_args_type = "path: string, count: number", ts_return_type = "Promise<DbConnection>")]
pub fn connect_db(env: Env, path: FastStr, count: u16) -> Result<JsObject> {
    let (defferred, promise) = env.create_deferred()?;

    tokio_workers::spawn(async move {
        let mut clients = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (client, connection) = match tokio_postgres::connect(&path.0, NoTls).await {
                Ok(res) => res,
                Err(_) => {
                    defferred.reject(Error::from_reason("Error connecting to the db."));
                    return;
                }
            };

            tokio_workers::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            clients.push(Arc::new(client));
        }

        defferred.resolve(|_| {
            Ok(DbConnection {
                clients,
                pos: AtomicUsize::new(0),
            })
        });
    });

    Ok(promise)
}

#[napi]
pub struct PreparedStatement {
    clients: Vec<(Arc<Client>, Statement)>,
    pos: AtomicUsize,
}

#[napi]
impl DbConnection {
    #[napi]
    pub fn query(&self, env: Env, query: FastStr) -> Result<JsObject> {
        let (defferred, promise) = env.create_deferred()?;
        let pos = self.pos.fetch_add(1, Relaxed) % self.clients.len();

        let client_copy = Arc::clone(&self.clients[pos]);
        tokio_workers::spawn(async move {
            let rows = client_copy.query(&query.0, &[]).await;

            match rows {
                Ok(res) => {
                    let resulting = PostgresRows(res);
                    defferred.resolve(|_| Ok(resulting))
                }
                Err(_) => defferred.reject(Error::from_reason("Failed to load rows.")),
            }
        });

        Ok(promise)
    }

    #[napi(ts_args_type = "query: string, count: number", ts_return_type = "Promise<PreparedStatement>")]
    pub fn prepare_statement(&self, env: Env, query: FastStr, count: u16) -> Result<JsObject> {
        let (defferred, promise) = env.create_deferred()?;

        let max = usize::min(count as usize, self.clients.len());
        let mut to_prepare_for = Vec::with_capacity(max);

        for i in 0..max {
            to_prepare_for.push(Arc::clone(&self.clients[i]))
        }

        tokio_workers::spawn(async move {
            let mut clients = Vec::with_capacity(max);

            for client in to_prepare_for.into_iter() {
                let prepared = match client.prepare(&query.0).await {
                    Ok(res) => res,
                    Err(_) => {
                        defferred.reject(Error::from_reason("Failed to load rows."));
                        return;
                    }
                };

                clients.push((client, prepared))
            }

            let resulting = PreparedStatement { clients, pos: AtomicUsize::new(0) };
            defferred.resolve(|_| Ok(resulting))
        });

        Ok(promise)
    }
}

#[napi]
impl PreparedStatement {
    #[napi]
    pub fn query(&self, env: Env) -> Result<JsObject> {
        let (defferred, promise) = env.create_deferred()?;
        let pos = self.pos.fetch_add(1, Relaxed) % self.clients.len();

        let pair = &self.clients[pos];

        let client = Arc::clone(&pair.0);
        let statement = pair.1.clone();

        tokio_workers::spawn(async move {
            let rows = client.query(&statement, &[]).await;

            match rows {
                Ok(res) => {
                    let resulting = PostgresRows(res);
                    defferred.resolve(|_| Ok(resulting))
                }
                Err(_) => defferred.reject(Error::from_reason("Failed to load rows.")),
            }
        });

        Ok(promise)
    }

}