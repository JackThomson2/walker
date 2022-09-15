use std::collections::HashMap;

use bytes::Bytes;
use napi::{bindgen_prelude::Buffer, Error, Result, Status};
use serde_json::Value;

use crate::{request::RequestBlob, Methods};

use super::response::JsResponse;

#[napi]
impl RequestBlob {
  #[inline]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_text(&self, response: String) -> Result<()> {
    let message = JsResponse::Text(Bytes::copy_from_slice(response.as_bytes()));
    self.oneshot.send(message).map_err(|_e| {
      Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
      )
    })
  }

  #[inline]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_object(&self, response: Value) -> Result<()> {
    let data = match serde_json::to_vec(&response) {
      Ok(res) => res,
      Err(_) => {
        return Err(Error::new(
          Status::GenericFailure,
          "Unable to send response".to_string(),
        ));
      }
    };
    
    let message = JsResponse::Text(Bytes::copy_from_slice(&data));
    self.oneshot.send(message).map_err(|_e| {
      Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
      )
    })
  }

  #[inline]
  #[napi]
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn get_params(&self) -> Option<HashMap<String, String>> {
    let method_str = self.data.method().to_uppercase();
    let method = match Methods::convert_from_str(&method_str) {
      Some(res) => res,
      None => {
        return None;
      }
    };

    crate::router::store::get_params(self.data.path(), method)
  }

  #[napi]
  /// Retrieve the raw body bytes in a Uint8Array to be used
  pub fn get_body(&self) -> Buffer {
    self.data.body().into()
  }
}
