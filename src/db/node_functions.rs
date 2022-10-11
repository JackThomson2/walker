use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::Result;
use tokio_postgres::Client;
use tokio_postgres::NoTls;

use crate::napi::fast_str::FastStr;
use crate::napi::postgres_rows::PostgresRows;
use crate::request::helpers::make_js_error;

#[napi]
pub struct DbConnection {
    clients: Vec<Arc<Client>>,
    pos: AtomicUsize
}

#[napi]
pub async fn connect_db(path: FastStr, count: u16) -> Result<DbConnection> {
    let mut clients = Vec::with_capacity(count as usize);
    
    for _ in 0..count {
      let (client, connection) = tokio_postgres::connect(&path.0, NoTls)
          .await
          .map_err(|_| make_js_error("Error connecting to db"))?;

      tokio::spawn(async move {
          if let Err(e) = connection.await {
              eprintln!("connection error: {}", e);
          }
      });

      clients.push(Arc::new(client));
    }

    Ok(DbConnection {
        clients,
        pos: AtomicUsize::new(0)
    })
}

#[napi]
impl DbConnection {
    #[napi]
    pub fn query(&self, env: Env, query: FastStr) -> Result<JsObject> {
        let (defferred, promise) = env.create_deferred()?;
        let pos = self.pos.fetch_add(1, Relaxed) % self.clients.len();

        let client_copy = Arc::clone(&self.clients[pos]);
        tokio::spawn(async move {
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
}
