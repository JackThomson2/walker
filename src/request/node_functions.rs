use std::collections::HashMap;

use actix_http::HttpMessage;
use bytes::{Bytes, BytesMut};
use napi::{bindgen_prelude::Uint8Array, Result};
use serde_json::Value;
use tera::{Context, Tera};

use crate::{request::RequestBlob, Methods};

use lazy_static::lazy_static;

use super::{
    helpers::{
        convert_header_map, load_body_from_payload, make_generic_error, split_and_get_query_params,
    },
    response::JsResponse,
    writer::Writer,
};

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

#[napi]
impl RequestBlob {
    #[inline(always)]
    fn send_result(&mut self, response: JsResponse) -> Result<()> {
        if self.sent {
            return Err(make_generic_error());
        }

        self.oneshot
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
        serde_json::to_writer(&mut Writer(&mut bytes), &response)
            .map_err(|_| make_generic_error())?;

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
        TEMPLATES
            .render_to(
                "users/profile.html",
                &Context::from_serialize(&data).map_err(|_| make_generic_error())?,
                &mut Writer(&mut buffer),
            )
            .map_err(|_| make_generic_error())?;

        let message = JsResponse::Template(buffer.freeze());
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// Get the query parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_query_params(&self) -> Option<HashMap<String, String>> {
        let query_string = self.data.uri().query()?.to_owned();

        Some(split_and_get_query_params(query_string))
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_url_params(&self) -> Option<HashMap<String, String>> {
      let method_str = self.data.method();
      let method = Methods::convert_from_actix(method_str.clone())?;
  
      crate::router::read_only::get_params(self.data.path(), method)
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn header_length(&self) -> i64 {
        let header_val = self.data.headers().len_keys();

        header_val as i64
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_header(&self, name: String) -> Option<String> {
        let header_val = self.data.headers().get(name)?;

        Some(header_val.to_str().ok()?.to_string())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_all_headers(&self) -> HashMap<String, String> {
        let header_val = self.data.headers();
        convert_header_map(header_val)
    }

    #[inline(always)]
    #[napi]
    /// Retrieve the raw body bytes in a Uint8Array to be used
    pub fn get_body(&mut self) -> Uint8Array {
        if let Some(body) = &self.body {
            return body.clone().into();
        }

        let bytes = load_body_from_payload(self.data.take_payload());
        self.body = Some(bytes.clone());

        bytes.into()
    }
}
