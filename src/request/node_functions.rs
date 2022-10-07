use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};
use futures::StreamExt;
use napi::{
  bindgen_prelude::{Uint8Array}, Error, Result, Status,
};
use serde_json::Value;

use crate::{request::RequestBlob, Methods};

use super::{response::JsResponse, writer::Writer};

#[napi]
impl RequestBlob {
  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_text(&mut self, response: String) {
    let message = JsResponse::Text(Bytes::copy_from_slice(response.as_bytes()));
    self.oneshot.set_close_on_send(true).send(message).now().ok();
    self.sent = true;
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_bytes_text(&mut self, response: Uint8Array) {
    let message = JsResponse::Text(Bytes::copy_from_slice(&response));
    self.oneshot.set_close_on_send(true).send(message).now().ok();
    self.sent = true;
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_object(&mut self, response: Value) -> Result<()> {
    let mut bytes = BytesMut::with_capacity(1024);

    if serde_json::to_writer(&mut Writer(&mut bytes), &response).is_err() {
      return Err(Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
      ));
    };

    let message = JsResponse::Json(bytes.freeze());

    self.oneshot.set_close_on_send(true).send(message).now().ok();
    self.sent = true;
    Ok(())
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_stringified_object(&mut self, response: String) {
    let message = JsResponse::Json(Bytes::copy_from_slice(response.as_bytes()));
    self.oneshot.set_close_on_send(true).send(message).now().ok();
    self.sent = true;
  }

  #[inline(always)]
  #[napi]
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn get_params(&self) -> Option<HashMap<String, String>> {
    let method_str = self.data.method().as_str().to_uppercase();
    let method = match Methods::convert_from_str(&method_str) {
      Some(res) => res,
      None => {
        return None;
      }
    };

    crate::router::read_only::get_params(self.data.path(), method)
  }

  #[inline(always)]
  #[napi]
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn header_length(&mut self) -> i64 {
    let header_val = self.data.headers_mut().len_keys();

    header_val as i64
  }

  #[inline(always)]
  #[napi]
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn get_header(&mut self, name: String) -> Option<String> {
    let header_val = self.data.headers_mut().get(name)?;

    Some(header_val.to_str().ok()?.to_string())
  }

  #[inline(always)]
  #[napi]
  /// Retrieve the raw body bytes in a Uint8Array to be used
  pub fn get_body(&mut self) -> Uint8Array {
    extreme::run(async move {
      let mut payload = self.data.take_payload();
      let mut bytes = BytesMut::new();

      while let Some(item) = payload.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
      }

      bytes.freeze().into()
    })
  }
}
