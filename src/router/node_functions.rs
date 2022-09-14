use napi::{bindgen_prelude::*, threadsafe_function::ThreadSafeCallContext};

use crate::{request::RequestBlob, router::store::add_new_route};

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
  pub fn from_str(method: &str) -> Option<Self> {
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

#[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
/// Use this to register a new route in the server, the callback function will be called
/// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
/// needed to get the information from the request
pub fn new_route(route: String, method: Methods, callback: JsFunction) -> Result<()> {
  let tsfn =
    callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Vec<RequestBlob>>| {
      Ok(ctx.value)
    })?;

  add_new_route(&route, method, tsfn)?;
  
  Ok(())
}
