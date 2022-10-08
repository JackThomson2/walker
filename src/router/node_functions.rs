use actix_http::Method;
use napi::bindgen_prelude::*;

use crate::{router::store::add_new_route, napi::tsfn::ThreadsafeFunction};

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

  #[inline(always)]
  pub fn convert_from_actix(method: Method) -> Option<Self> {
    match method {
        Method::GET => Some(Methods::GET),
        Method::POST => Some(Methods::POST),
        Method::PUT => Some(Methods::PUT),
        Method::PATCH => Some(Methods::PATCH),
        Method::DELETE => Some(Methods::DELETE),
        _ => None
    }
  }
}

#[cold]
#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Use this to register a new route in the server, the callback function will be called
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn new_route(route: String, method: Methods, callback: JsFunction) -> Result<()> {
  let tsfn = ThreadsafeFunction::create(callback.0.env, callback.0.value, 1024)?;

  add_new_route(&route, method, tsfn)
}

#[cold]
#[napi(ts_args_type = "route: string, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a GET request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn get(route: String, callback: JsFunction) -> Result<()> {
  new_route(route, Methods::GET, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a POST request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn post(route: String, callback: JsFunction) -> Result<()> {
  new_route(route, Methods::POST, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a PUT request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn put(route: String, callback: JsFunction) -> Result<()> {
  new_route(route, Methods::PUT, callback)
}

#[cold]
#[napi(ts_args_type = "route: string, callback: (result: RequestBlob) => void")]
/// Adds a handler for the a PATCH request
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn patch(route: String, callback: JsFunction) -> Result<()> {
  new_route(route, Methods::PATCH, callback)
}
