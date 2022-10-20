use std::mem::{self, MaybeUninit};

use actix_http::{HttpMessage, Request};
use bytes::{BufMut, Bytes, BytesMut};
use napi::{bindgen_prelude::Uint8Array, sys, Result};
use serde_json::Value;
use tokio::sync::oneshot::Sender;

use crate::{
    napi::{
        buff_str::BuffStr, bytes_recv::JsBytes, fast_serde::FasterValue, fast_str::FastStr,
        halfbrown::HalfBrown,
    },
    router,
};

use super::{
    helpers::{
        convert_header_map, load_body_from_payload, make_js_error, split_and_get_query_params,
    },
    response::{InnerResp, JsResponse},
};

#[napi]
pub struct RequestBlob {
    pub(crate) data: MaybeUninit<Request>,
    pub(crate) oneshot: MaybeUninit<Sender<JsResponse>>,
    pub(crate) sent: bool,
    pub(crate) body: Option<Bytes>,
    pub(crate) headers: MaybeUninit<Option<Vec<(Bytes, Bytes)>>>,
    pub(crate) written: usize,
}

#[napi]
impl RequestBlob {
    pub fn new_empty_with_js(js_obj: sys::napi_value) -> Box<Self> {
        let oneshot = MaybeUninit::uninit();
        let headers = MaybeUninit::uninit();
        let data = MaybeUninit::uninit();

        Box::new(Self {
            data,
            oneshot,
            sent: false,
            body: None,
            headers,
            written: 0,
        })
    }

    #[inline]
    pub fn new_with_route(data: Request, sender: Sender<JsResponse>) -> Self {
        let oneshot = MaybeUninit::new(sender);
        let headers = MaybeUninit::new(None);
        let data = MaybeUninit::new(data);

        Self {
            data,
            oneshot,
            sent: false,
            body: None,
            headers,
            written: 0,
        }
    }

    #[inline]
    pub fn store_self_data(&mut self, data: Request, sender: Sender<JsResponse>) {
        let oneshot = MaybeUninit::new(sender);
        let headers = MaybeUninit::new(None);
        let data = MaybeUninit::new(data);

        if self.written > 0 {
            unsafe {
                self.data.assume_init_drop();
            }
        }

        self.data = data;
        self.oneshot = oneshot;
        self.headers = headers;
        self.body = None;
        self.sent = false;
        self.written += 1;
    }

    #[inline(always)]
    fn get_data_val<'a>(&'a self) -> &'a Request {
        unsafe { self.data.assume_init_ref() }
    }

    #[inline(always)]
    fn get_data_val_mut(&mut self) -> &mut Request {
        unsafe { self.data.assume_init_mut() }
    }

    #[inline(always)]
    pub fn send_result_checked(&mut self, inner: InnerResp, checked: bool) -> Result<()> {
        if checked && self.sent {
            return Err(make_js_error("Already sent response."));
        }

        self.sent = true;
        let oneshot = unsafe {
            let result = std::mem::replace(&mut self.oneshot, MaybeUninit::uninit());
            result.assume_init()
        };

        let headers = unsafe {
            let result = std::mem::replace(&mut self.headers, MaybeUninit::uninit());
            result.assume_init()
        };

        let js_resp = JsResponse { inner, headers };
        let res = oneshot.send(js_resp);

        if checked && res.is_err() {
            eprintln!("Error sending response, the reciever may have dropped.");
        }

        Ok(())
    }

    #[inline(always)]
    pub fn send_result(&mut self, response: InnerResp) -> Result<()> {
        self.send_result_checked(response, true)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_text(&mut self, response: BuffStr) -> Result<()> {
        let message = InnerResp::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_fast_text(&mut self, response: FastStr) -> Result<()> {
        let message = InnerResp::Text(Bytes::from_iter(response.0.into_bytes()));
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_napi_text(&mut self, response: String) -> Result<()> {
        let message = InnerResp::Text(Bytes::from_iter(response.into_bytes()));
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
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_empty_text(&mut self) -> Result<()> {
        let message = InnerResp::EmptyString;
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn unchecked_send_empty_text(&mut self) {
        let message = InnerResp::EmptyString;
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    pub fn send_object(&mut self, response: Value) -> Result<()> {
        let bytes = BytesMut::with_capacity(128);
        let mut writer = bytes.writer();
        serde_json::to_writer(&mut writer, &response)
            .map_err(|_| make_js_error("Error serialising data."))?;

        let bytes = writer.into_inner();
        let message = InnerResp::Json(bytes.freeze());
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This needs to be a key value object, any other is undefined behaviour
    pub fn send_fast_object(&mut self, response: FasterValue) -> Result<()> {
        let bytes = BytesMut::with_capacity(128);
        let mut writer = bytes.writer();
        serde_json::to_writer(&mut writer, &response.0)
            .map_err(|_| make_js_error("Error serialising data."))?;

        let bytes = writer.into_inner();
        let message = InnerResp::Json(bytes.freeze());
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
    pub fn get_body(&mut self) -> Result<Uint8Array> {
        if let Some(body) = &self.body {
            return Ok(body.clone().into());
        }

        let mut_data = self.get_data_val_mut();

        let bytes = load_body_from_payload(mut_data.take_payload())?;
        self.body = Some(bytes.clone());

        Ok(bytes.into())
    }
}
