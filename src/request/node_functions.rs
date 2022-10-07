use std::collections::HashMap;

use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use napi::{bindgen_prelude::Uint8Array, Error, Result, Status};
use serde_json::Value;
use tera::{Context, Tera};

use crate::request::RequestBlob;

use lazy_static::lazy_static;

use super::{response::JsResponse, writer::Writer};


lazy_static! {
  pub static ref TEMPLATES: Tera = {
      let tera = match Tera::new("templates/**/*.html") {
          Ok(t) => t,
          Err(e) => {
              println!("Parsing error(s): {}", e);
              ::std::process::exit(1);
          }
      };
      tera
  };
}


#[cold]
#[inline(never)]
fn make_generic_error() -> Error {
  Error::new(
    Status::GenericFailure,
    "Unable to send response".to_string(),
  )
}

#[napi]
impl RequestBlob {
  #[inline(always)]
  fn send_result(&mut self, response: JsResponse) -> Result<()> {
    if self.sent {
      return Err(make_generic_error());
    }

    self
      .oneshot
      .set_close_on_send(true)
      .send(response)
      .now()
      .map_err(|_| make_generic_error())?;
    self.sent = true;

    Ok(())
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_text(&mut self, response: String) -> Result<()> {
    let message = JsResponse::Text(Bytes::copy_from_slice(response.as_bytes()));
    self.send_result(message)
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_bytes_text(&mut self, response: Uint8Array) -> Result<()> {
    let message = JsResponse::Text(Bytes::copy_from_slice(&response));
    self.send_result(message)
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_object(&mut self, response: Value) -> Result<()> {
    let mut bytes = BytesMut::with_capacity(1024);
    serde_json::to_writer(&mut Writer(&mut bytes), &response).map_err(|_| make_generic_error())?;

    let message = JsResponse::Json(bytes.freeze());
    self.send_result(message)
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_stringified_object(&mut self, response: String) -> Result<()> {
    let message = JsResponse::Json(Bytes::copy_from_slice(response.as_bytes()));
    self.send_result(message)
  }

  #[inline(always)]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn send_template_resp(&mut self, data: Value) -> Result<()> {
    let mut buffer = BytesMut::with_capacity(2048);
    TEMPLATES.render_to(
      "users/profile.html",
      &Context::from_serialize(&data).map_err(|_| make_generic_error())?,
      &mut Writer(&mut buffer),
    ).map_err(|_| make_generic_error())?;

    let message = JsResponse::Template(buffer.freeze());
    self.send_result(message)
  }

  #[inline(always)]
  #[napi]
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn get_params(&self) -> Option<HashMap<String, String>> {
    let mut params = HashMap::with_capacity(16);
    let query_string = self.data.uri().query()?.to_owned();

    for pair in query_string.split('&') {
      let mut split_two = pair.split('=');

      let key = match split_two.next() {
        Some(res) => res,
        None => continue,
      };

      let val = match split_two.next() {
        Some(res) => res,
        None => continue,
      };

      params.insert(key.to_string(), val.to_string());
    }
    Some(params)
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
  /// Get the url parameters as an object with each key and value
  /// this will only be null if an error has occurred
  pub fn get_all_headers(&mut self) -> HashMap<String, String> {
    let header_val = self.data.headers_mut();
    let mut return_map = HashMap::with_capacity(header_val.len());

    for (key, value) in header_val.iter() {
      let string_val = match value.to_str() {
        Ok(res) => res,
        Err(_) => continue,
      };

      return_map.insert(key.to_string(), string_val.to_string());
    }

    return_map
  }

  #[inline(always)]
  #[napi]
  /// Retrieve the raw body bytes in a Uint8Array to be used
  pub fn get_body(&mut self) -> Uint8Array {
    if let Some(body) = &self.body {
      return body.clone().into();
    }

    extreme::run(async move {
      let mut payload = self.data.take_payload();
      let mut bytes = BytesMut::with_capacity(1024);

      while let Some(item) = payload.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
      }

      let bytes = bytes.freeze();
      self.body = Some(bytes.clone());

      bytes.into()
    })
  }
}
