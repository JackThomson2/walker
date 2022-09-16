use std::sync::{atomic::AtomicUsize, Arc};

use may_postgres::{Client, Row, SimpleQueryRow, SimpleQueryMessage};
use napi::{bindgen_prelude::*, JsString};
use serde_json::Value;

#[napi]
pub struct DbPool {
  idx: AtomicUsize,
  clients: Vec<Arc<Client>>,
  number: usize,
}

fn create_client(url: &str) -> Result<Client> {
  may_postgres::connect(url).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      e.to_string(),
    )
  })
}

#[napi]
impl DbPool {
  #[napi]
  pub fn new(url: String, number: i64) -> Result<Self> {
    let number = (number as usize).next_power_of_two();
    let mut clients = Vec::with_capacity(number);
    for _ in 0..number {
      let client = create_client(&url)?;
      clients.push(Arc::new(client));
    }

    Ok(DbPool {
      idx: AtomicUsize::new(0),
      clients,
      number,
    })
  }

  fn get_next_client(&self) -> Arc<Client> {
    let idx = self.idx.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    unsafe { self.clients.get_unchecked(idx & (self.number - 1)).clone() }
  }

  #[napi]
  pub fn query(&self, input: String) -> AsyncTask<DbGetter> {
    AsyncTask::new(DbGetter {
      client: self.get_next_client(),
      input,
    })
  }
}

pub struct DbGetter {
  client: Arc<Client>,
  input: String,
}

impl DbGetter {
  fn queryClient(&self) -> Vec<Vec<String>> {

    let res = self.client.simple_query(&self.input).unwrap();

    let mut resulting: Vec<Vec<String>> = Vec::with_capacity(res.len());

    for i in res {
      match i {
        SimpleQueryMessage::Row(res) =>  {
          let mut adding = Vec::with_capacity(res.len());

          for i in 0..res.len() {
            adding.push(res.get(i).unwrap().to_string());
          }

          resulting.push(adding);
        },
        _ => {}
      }
    }

    resulting
  }
}

#[napi]
impl Task for DbGetter {
  type Output = Vec<Vec<String>>;
  type JsValue = Vec<Vec<String>>;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(self.queryClient())
  }

  fn resolve(&mut self, _: Env, output: Vec<Vec<String>>) -> Result<Self::JsValue> {
    Ok(output)
  }
}
