use napi::bindgen_prelude::*;

use crate::router::store::add_new_route;

use super::hackery::convert_js_func_to_shared_ref;

#[napi]
/// The different HTTP methods 
pub enum Methods {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}

impl Methods {
  #[inline(always)]
  pub fn convert_from_str(method: &str) -> Option<Self> {
    match method {
        "GET" => Some(Methods::GET),
        "POST" => Some(Methods::POST),
        "PUT" => Some(Methods::PUT),
        "PATCH" => Some(Methods::PATCH),
        "DELETE" => Some(Methods::DELETE),
        _ => None
    }
  }
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Use this to register a new route in the server, the callback function will be called
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn new_route(env: Env, route: String, method: Methods, callback: JsFunction) -> Result<()> {
  let shared_func_ref = unsafe { convert_js_func_to_shared_ref(&env, callback) };

  add_new_route(&route, method, shared_func_ref)?;
  
  Ok(())
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a GET request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn get(env: Env, route: String, callback: JsFunction) -> Result<()> {
  new_route(env, route, Methods::GET, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a POST request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn post(env: Env, route: String, callback: JsFunction) -> Result<()> {
  new_route(env, route, Methods::POST, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a PUT request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn put(env: Env, route: String, callback: JsFunction) -> Result<()> {
  new_route(env, route, Methods::PUT, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a PATCH request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn patch(env: Env, route: String, callback: JsFunction) -> Result<()> {
  new_route(env, route, Methods::PATCH, callback)
}
