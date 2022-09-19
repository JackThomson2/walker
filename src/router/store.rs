use std::collections::HashMap;

use matchit::{Params, Router};
use napi::bindgen_prelude::*;
use usync::RwLock;

use lazy_static::lazy_static;

use crate::{types::CallBackFunction, Methods};

type ThreadSafeLookup = RwLock<Router<CallBackFunction>>;

lazy_static! {
  static ref GLOBAL_DATA: InternalRoutes = InternalRoutes::new_manager();
}


struct InternalRoutes {
  get: ThreadSafeLookup,
  post: ThreadSafeLookup,
  put: ThreadSafeLookup,
  patch: ThreadSafeLookup,
  delete: ThreadSafeLookup,
}

impl InternalRoutes {
  #[inline]
  fn new_manager() -> Self {
    Self {
      get: RwLock::new(Router::new()),
      post: RwLock::new(Router::new()),
      put: RwLock::new(Router::new()),
      patch: RwLock::new(Router::new()),
      delete: RwLock::new(Router::new()),
    }
  }

  #[inline]
  fn get_rw_from_method(&self, method: Methods) -> &ThreadSafeLookup {
    match method {
      Methods::GET => &self.get,
      Methods::POST => &self.post,
      Methods::PUT => &self.put,
      Methods::PATCH => &self.patch,
      Methods::DELETE => &self.delete,
    }
  }
}

#[inline]
pub fn add_new_route(route: &str, method: Methods, function: CallBackFunction) -> Result<()> {
  let lock = GLOBAL_DATA.get_rw_from_method(method);
  let mut writing = lock.write();

  writing
    .insert(route, function)
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
}

#[inline]
pub fn get_route(route: &str, method: Methods) -> Option<CallBackFunction> {
  let lock = GLOBAL_DATA.get_rw_from_method(method);
  let checking = lock.read();
  let found = checking.at(route);

  match found {
    Ok(res) => Some(res.value.clone()),
    Err(_) => None,
  }
}

#[inline]
fn params_to_map(params: &Params) -> HashMap<String, String> {
  let mut map = HashMap::with_capacity(params.len());

  for (key, value) in params.iter() {
    map.insert(key.to_string(), value.to_string());
  }

  map
}

#[inline]
pub fn get_params(route: &str, method: Methods) -> Option<HashMap<String, String>> {
  let lock = GLOBAL_DATA.get_rw_from_method(method);
  let checking = lock.read();
  let found = checking.at(route);

  match found {
    Ok(res) => Some(params_to_map(&res.params)),
    Err(_) => None,
  }
}
