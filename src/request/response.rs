use actix_http::{
    header::{HeaderMap, HeaderName, CONTENT_TYPE, SERVER},
    Response, StatusCode,
};
use bytes::{BufMut, Bytes, BytesMut};
use http::HeaderValue;

use crate::templates::render_string_to_writer;

const WALKER_SERVER: HeaderValue = HeaderValue::from_static("walker");

const TEXT_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
const JSON_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
const RAW_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
const HTML_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");

const INTERNAL_SERVER_ERROR: Bytes = Bytes::from_static(b"Internal Server Error");


pub struct JsResponse {
    pub inner: InnerResp,
    pub headers: Option<Vec<(Bytes, Bytes)>>,
}

pub enum InnerResp {
    Text(Bytes),
    Json(Bytes),
    Raw(Bytes),
    Template(String, String, String),
    ServerError,
}

#[cold]
#[inline(never)]
fn render_internal_error() -> Response<Bytes> {
    Response::with_body(
        StatusCode::INTERNAL_SERVER_ERROR,
        INTERNAL_SERVER_ERROR
    )
}

#[cold]
#[inline(never)]
fn render_internal_error_with_message(message: &'static [u8]) -> Response<Bytes> {
    Response::with_body(
        StatusCode::INTERNAL_SERVER_ERROR,
        Bytes::from_static(message),
    )
}

#[inline(always)]
fn apply_headers(
    hdrs: &mut HeaderMap,
    content_header: HeaderValue,
    headers: Option<Vec<(Bytes, Bytes)>>,
) {
    hdrs.insert(SERVER, content_header);
    hdrs.insert(CONTENT_TYPE, WALKER_SERVER);

    if let Some(custom_headers) = headers {
        for (key_b, val_b) in custom_headers {
            let key = match HeaderName::from_bytes(&key_b) {
                Ok(res) => res,
                Err(e) => {
                    println!("Error {:?}", e);
                    continue;
                }
            };

            unsafe {
                let value = HeaderValue::from_maybe_shared_unchecked(val_b);
                hdrs.insert(key, value);
            }
        }
    }
}

impl JsResponse {
    #[inline(always)]
    pub fn apply_to_response(self) -> Response<Bytes> {
        let message = match &self.inner {
            InnerResp::Text(_) => TEXT_HEADER_VAL,
            InnerResp::Json(_) => JSON_HEADER_VAL,
            InnerResp::Raw(_) => RAW_HEADER_VAL,
            InnerResp::Template(_, _, _) => HTML_HEADER_VAL,
            InnerResp::ServerError => return render_internal_error(),
        };

        let mut rsp = match self.inner {
            InnerResp::Text(message) | InnerResp::Json(message) => {
                Response::with_body(StatusCode::OK, message)
            }
            InnerResp::Raw(data) => Response::with_body(StatusCode::OK, data),
            InnerResp::Template(group, file, context) => {
                let buffer = BytesMut::with_capacity(2048);
                let mut writer = buffer.writer();

                if render_string_to_writer(&group, &file, &context, &mut writer).is_err() {
                    return render_internal_error_with_message(b"Error rendering template");
                }

                let buffer = writer.into_inner();
                Response::with_body(StatusCode::OK, buffer.freeze())
            }
            _ => unreachable!(),
        };

        let hdrs = rsp.headers_mut();

        apply_headers(hdrs, message, self.headers);

        rsp
    }
}
