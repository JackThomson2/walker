use matchit::Router;
use napi::bindgen_prelude::*;
use once_cell::sync::Lazy;
use usync::RwLock;

use crate::types::CallBackFunction;

type ThreadSafeLookup = RwLock<Router<CallBackFunction>>;

static GLOBAL_DATA: Lazy<ThreadSafeLookup> = Lazy::new(|| RwLock::new(Router::new()));

#[inline]
pub fn add_new_route(route: &str, function: CallBackFunction) -> Result<()> {
  let mut writing = GLOBAL_DATA.write();

  writing
    .insert(route, function)
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
}

pub fn get_route(route: &str) -> Option<CallBackFunction> {
    let checking = GLOBAL_DATA.read();
    let found = checking.at(route);

    match found {
        Ok(res) => Some(res.value.clone()),
        Err(_) => return None
    }
}