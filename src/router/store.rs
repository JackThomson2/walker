use std::{collections::HashMap, sync::Arc};

use matchit::{Params, Router};
use usync::RwLock;

use crate::{types::CallBackFunction, Methods};

use super::builder::ServerBuilder;

pub type RouterType = Router<CallBackFunction>;

pub type ThreadSafeLookup = RwLock<RouterType>;
pub type LookupRef = Arc<RouterType>;

#[derive(Clone)]
pub struct InternalRoutes {
  get: LookupRef,
  post: LookupRef,
  put: LookupRef,
  patch: LookupRef,
  delete: LookupRef,
}

fn lock_to_arc(incoming: &ThreadSafeLookup) -> LookupRef {
  let reading = incoming.read();
  Arc::new(reading.clone())
}

#[inline]
fn params_to_map(params: &Params) -> HashMap<String, String> {
  let mut map = HashMap::with_capacity(params.len());

  for (key, value) in params.iter() {
    map.insert(key.to_string(), value.to_string());
  }

  map
}

impl InternalRoutes {
  #[inline]
  pub fn new_manager(builder: &ServerBuilder) -> Self {
    Self {
      get: lock_to_arc(&builder.get),
      post: lock_to_arc(&builder.post),
      put: lock_to_arc(&builder.put),
      patch: lock_to_arc(&builder.patch),
      delete: lock_to_arc(&builder.delete),
    }
  }

  #[inline]
  fn get_ref_from_method(&self, method: Methods) -> &LookupRef {
    match method {
      Methods::GET => &self.get,
      Methods::POST => &self.post,
      Methods::PUT => &self.put,
      Methods::PATCH => &self.patch,
      Methods::DELETE => &self.delete,
    }
  }

  #[inline]
  pub fn get_route(&self, route: &str, method: Methods) -> Option<CallBackFunction> {
    let checking = self.get_ref_from_method(method);
    let found = checking.at(&route);

    match found {
      Ok(res) => Some(res.value.clone()),
      Err(_) => None,
    }
  }

  #[inline]
  pub fn get_params(&self, route: String, method: Methods) -> Option<HashMap<String, String>> {
    let checking = self.get_ref_from_method(method);
    let found = checking.at(&route);

    match found {
      Ok(res) => Some(params_to_map(&res.params)),
      Err(_) => None,
    }
  }
}
