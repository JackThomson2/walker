use bytes::Bytes;
use crate::minihttp::Response;
use napi::bindgen_prelude::Buffer;

pub enum JsResponse {
    Text(Bytes),
    Json(Bytes),
    TextBuffer(Buffer),
    Raw(Bytes)
}

impl JsResponse {
    #[inline(always)]
    fn apply_headers(&self, rsp: &mut Response) {
        let message = match self {
            Self::Text(_) | Self::TextBuffer(_)=> "Content-Type: text/plain",
            Self::Json(_) => "Content-Type: application/json",
            Self::Raw(_) => "Content-Type: application/octet-stream",
        };

        rsp.header(message);
    }

    #[inline(always)]
    fn apply_response(&self, rsp: &mut Response) {
        match self {
            Self::Text(message) | Self::Json(message) => {
                rsp.write_bytes(message);
            },
            Self::Raw(data) => {
                rsp.write_bytes(data);
            },
            Self::TextBuffer(buf) => {
                rsp.write_buffer(buf);
            }
        }
    }

    #[inline(always)]
    pub fn apply_to_response(&self, rsp: &mut Response) {
        self.apply_headers(rsp);
        self.apply_response(rsp);
    }
}