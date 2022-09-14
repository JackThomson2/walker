use std::collections::HashMap;

use matchit::{Router, Params};
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
        Ok(res) => {
            Some(res.value.clone())
        },
        Err(_) => None
    }
}

fn params_to_map(params: &Params) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(params.len());

    for (key, value) in params.iter() {
        map.insert(key.to_string(), value.to_string());
    }

    map
}

pub fn get_params(route: &str) -> Option<HashMap<String, String>> {
    let checking = GLOBAL_DATA.read();
    let found = checking.at(route);

    match found {
        Ok(res) => {
            Some(params_to_map(&res.params))
        },
        Err(_) => None
    }
}