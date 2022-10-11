use std::mem::MaybeUninit;

use actix_http::HttpMessage;
use bytes::{Bytes, BytesMut};
use napi::{bindgen_prelude::Uint8Array, Result};
use serde_json::Value;

use crate::{
    napi::{buff_str::BuffStr, bytes_recv::JsBytes, fast_str::FastStr, halfbrown::HalfBrown},
    request::RequestBlob,
    Methods,
};

use super::{
    helpers::{
        convert_header_map, load_body_from_payload, make_js_error,
        split_and_get_query_params,
    },
    response::JsResponse,
    writer::Writer,
};

#[napi]
impl RequestBlob {
    #[inline(always)]
    fn send_result(&mut self, response: JsResponse) -> Result<()> {
        if self.sent {
            return Err(make_js_error("Already sent response."));
        }

        self.sent = true;

        let oneshot = unsafe {
            let result = std::mem::replace(&mut self.oneshot, MaybeUninit::uninit());
            result.assume_init()
        };

        oneshot
            .send(response)
            .map_err(|_| make_js_error("Error sending response."))?;

        Ok(())
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_text(&mut self, response: BuffStr) -> Result<()> {
        let message = JsResponse::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_fast_text(&mut self, response: FastStr) -> Result<()> {
        let message = JsResponse::Text(Bytes::from_iter(response.0.into_bytes()));
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_napi_text(&mut self, response: String) -> Result<()> {
        let message = JsResponse::Text(Bytes::from_iter(response.into_bytes()));
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_bytes_text(&mut self, response: JsBytes) -> Result<()> {
        let message = JsResponse::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_object(&mut self, response: Value) -> Result<()> {
        let mut bytes = BytesMut::with_capacity(1024);
        serde_json::to_writer(&mut Writer(&mut bytes), &response)
            .map_err(|_| make_js_error("Error serialising data."))?;

        let message = JsResponse::Json(bytes.freeze());
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_stringified_object(&mut self, response: BuffStr) -> Result<()> {
        let message = JsResponse::Json(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_template_resp(
        &mut self,
        group_name: FastStr,
        file_name: FastStr,
        context_json: FastStr,
    ) -> Result<()> {
        let message = JsResponse::Template(group_name.0, file_name.0, context_json.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// Get the query parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_query_params(&self) -> Option<HalfBrown<String, String>> {
        let query_string = self.data.uri().query()?.to_owned();

        Some(split_and_get_query_params(query_string))
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_url_params(&self) -> Option<HalfBrown<String, String>> {
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
    pub fn get_header(&self, name: FastStr) -> Option<String> {
        let header_val = self.data.headers().get(name.0)?;

        Some(header_val.to_str().ok()?.to_string())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_all_headers(&self) -> HalfBrown<String, String> {
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
