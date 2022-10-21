use actix_http::HttpMessage;
use bytes::Bytes;
use napi::{bindgen_prelude::Uint8Array, Result};
use serde_json::Value;

use crate::{
    napi::{
        buff_str::BuffStr, bytes_recv::JsBytes, fast_serde::FasterValue, fast_str::FastStr,
        halfbrown::HalfBrown,
    },
    router, response::InnerResp,
};

use super::{
    helpers::{
        convert_header_map, split_and_get_query_params, value_to_bytes,
    },
    RequestBlob
};

#[napi]
impl RequestBlob {
    #[inline(always)]
    #[napi(ts_args_type = "response: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_text(&mut self, response: BuffStr) -> Result<()> {
        let message = InnerResp::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_fast_text(&mut self, response: FastStr) -> Result<()> {
        let message = InnerResp::Text(Bytes::from_iter(response.0.into_bytes()));
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_bytes_text(&mut self, response: JsBytes) -> Result<()> {
        let message = InnerResp::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn unchecked_send_bytes_text(&mut self, response: JsBytes) {
        let message = InnerResp::Text(response.0);
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_empty_text(&mut self) -> Result<()> {
        let message = InnerResp::EmptyString;
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn unchecked_send_empty_text(&mut self) {
        let message = InnerResp::EmptyString;
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_object(&mut self, response: Value) -> Result<()> {
        let message = InnerResp::Json(value_to_bytes(response)?);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: any")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This needs to be a key value object, any other is undefined behaviour
    pub fn send_fast_object(&mut self, response: FasterValue) -> Result<()> {
        let message = InnerResp::Json(value_to_bytes(response.0)?);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_stringified_object(&mut self, response: BuffStr) -> Result<()> {
        let message = InnerResp::Json(response.0);
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
        let message = InnerResp::Template(group_name.0, file_name.0, context_json.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// Get the query parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_query_params(&self) -> Option<HalfBrown<String, String>> {
        let query_string = self.get_data_val().uri().query()?.to_owned();
        Some(split_and_get_query_params(query_string))
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_url_params(&self) -> Option<HalfBrown<String, String>> {
        let method_str = self.get_data_val().method();
        router::read_only::get_params(self.get_data_val().path(), method_str.clone())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn header_length(&self) -> i64 {
        let header_val = self.get_data_val().headers().len_keys();

        header_val as i64
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_header(&self, name: FastStr) -> Option<String> {
        let header_val = self.get_data_val().headers().get(name.0)?;

        Some(header_val.to_str().ok()?.to_string())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_all_headers(&self) -> HalfBrown<String, String> {
        let header_val = self.get_data_val().headers();
        convert_header_map(header_val)
    }

    #[inline(always)]
    #[napi]
    /// Add a new header to the response sent to the user
    pub fn add_header(&mut self, key: BuffStr, value: BuffStr) {
        if self.sent {
            return;
        }

        let headers = unsafe { self.headers.assume_init_mut() };

        if let Some(list_of_headers) = headers {
            list_of_headers.push((key.0, value.0))
        } else {
            *headers = Some(vec![(key.0, value.0)])
        }
    }

    #[inline(always)]
    #[napi]
    /// Retrieve the raw body bytes in a Uint8Array to be used
    pub fn get_body(&mut self) -> Uint8Array {
        match &self.body {
            Some(res) => res.clone().into(),
            None => vec![].into()
        }
    }
}
