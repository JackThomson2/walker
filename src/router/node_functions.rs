use napi::bindgen_prelude::*;


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