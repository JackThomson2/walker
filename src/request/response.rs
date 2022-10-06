use bytes::Bytes;
use actix_http::{Response, StatusCode, header::{CONTENT_TYPE, SERVER}};
use http::HeaderValue;
use napi::bindgen_prelude::Buffer;

pub enum JsResponse {
    Text(Bytes),
    Json(Bytes),
    TextBuffer(Buffer),
    Raw(Bytes)
}

impl JsResponse {
    #[inline(always)]
    fn apply_headers(&self, rsp: &mut Response<Bytes>) {
        let message = match self {
            Self::Text(_) | Self::TextBuffer(_)=> "Content-Type: text/plain",
            Self::Json(_) => "Content-Type: application/json",
            Self::Raw(_) => "Content-Type: application/octet-stream",
        };

        let header = HeaderValue::from_static(message);
        let server = HeaderValue::from_static("walker");

        let hdrs = rsp.headers_mut();
        hdrs.insert(SERVER, server);
        hdrs.insert(CONTENT_TYPE, header);
    }

    #[inline(always)]
    fn apply_response(&self) -> Response<Bytes> {
        match self {
            Self::Text(message) | Self::Json(message) => {
                Response::with_body(StatusCode::OK, message.clone())
            },
            Self::Raw(data) => {
                Response::with_body(StatusCode::OK, data.clone())
            },
            Self::TextBuffer(buf) => {
                Response::with_body(StatusCode::OK, Bytes::copy_from_slice(buf))
            }
        }
    }

    #[inline(always)]
    pub fn apply_to_response(&self) -> Response<Bytes> {
        let mut res = self.apply_response();
        self.apply_headers(&mut res);

        res
    }
}