use std::sync::Arc;
use std::time::Duration;

use actix_rt::time::sleep;
use futures::prelude::*;
use napi::JsDeferred;
use napi::JsObject;
use napi::bindgen_prelude::*;
use napi::Result;
use tokio_postgres::Client;
use tokio_postgres::NoTls;

use crate::napi::fast_str::FastStr;
use crate::napi::postgres::PostgresData;
use crate::request::helpers::make_js_error;

#[napi]
pub struct DbConnection {
    client: Arc<Client>,
}

#[napi]
pub async fn connect_db(path: FastStr) -> Result<DbConnection> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&path.0, NoTls)
        .await
        .map_err(|_| make_js_error("Error connecting to db"))?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(DbConnection { client: Arc::new(client) })
}

#[napi]
impl DbConnection {

  #[napi]
  pub fn read_file_async(&self, env: Env) -> Result<JsObject> {
      let (defferred, promise) = env.create_deferred()?;
  
      tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        defferred.resolve(|env| {
          env.create_int32(1)
        })
      });
  
      Ok(promise)
  }  
}

