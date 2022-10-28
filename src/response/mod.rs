use actix_http::{
    header::{HeaderMap, HeaderName, CONTENT_TYPE, SERVER},
    Response, StatusCode,
};
use bytes::Bytes;
use http::HeaderValue;

use crate::templates::store_in_bytes_buffer;

static WALKER_SERVER: HeaderValue = HeaderValue::from_static("walker");

static TEXT_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
static JSON_HEADER_VAL: HeaderValue = HeaderValue::from_static("application/json; charset=UTF-8");
static RAW_HEADER_VAL: HeaderValue = HeaderValue::from_static("application/octet-stream; charset=UTF-8");
static HTML_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/html; charset=UTF-8");

static INTERNAL_SERVER_ERROR: Bytes = Bytes::from_static(b"Internal Server Error");

pub struct JsResponse {
    pub inner: InnerResp,
    pub status_code: Option<u16>,
    pub headers: Option<Vec<(Bytes, Bytes)>>,
}

pub enum InnerResp {
    Text(Bytes),
    Json(Bytes),
    Raw(Bytes),
    Template(String, String, String),
    ServerError,
    ServerErrorWithMessage(Bytes),
    EmptyString,
}

use InnerResp::*;

#[cold]
#[inline(never)]
fn render_internal_error() -> Response<Bytes> {
    Response::with_body(
        StatusCode::INTERNAL_SERVER_ERROR,
        INTERNAL_SERVER_ERROR.clone(),
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

#[cold]
#[inline(never)]
fn render_internal_error_with_bytes(message: Bytes) -> Response<Bytes> {
    Response::with_body(
        StatusCode::INTERNAL_SERVER_ERROR,
        message,
    )
}

#[inline(always)]
fn apply_headers(
    hdrs: &mut HeaderMap,
    content_header: HeaderValue,
    headers: Option<Vec<(Bytes, Bytes)>>,
) {
    hdrs.insert(SERVER, WALKER_SERVER.clone());
    hdrs.insert(CONTENT_TYPE, content_header);

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
    #[cold]
    #[inline(never)]
    fn convert_to_status_code(code: u16) -> StatusCode {
        StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[inline(always)]
    fn get_status_code(status: Option<u16>) -> StatusCode {
        match status {
            Some(code) => Self::convert_to_status_code(code),
            None => StatusCode::OK,
        }
    }

    #[inline(always)]
    pub fn apply_to_response(self) -> Response<Bytes> {
        let message = match &self.inner {
            Text(_) | EmptyString => TEXT_HEADER_VAL.clone(),
            Json(_) => JSON_HEADER_VAL.clone(),
            Raw(_) => RAW_HEADER_VAL.clone(),
            Template(_, _, _) => HTML_HEADER_VAL.clone(),
            ServerError => return render_internal_error(),
            ServerErrorWithMessage(message) => return render_internal_error_with_bytes(message.clone()),
        };

        let bytes = match self.inner {
            Text(message) | Json(message) | Raw(message) => message,
            Template(group, file, context) => {
                let buffer = match store_in_bytes_buffer(&group, &file, &context) {
                    Ok(res) => res,
                    Err(_) => return render_internal_error_with_message(b"Error rendering template")
                };
                buffer.freeze()
            }
            EmptyString => Bytes::new(),
            _ => unreachable!(),
        };

        let mut rsp = Response::with_body(Self::get_status_code(self.status_code), bytes);
        let hdrs = rsp.headers_mut();

        apply_headers(hdrs, message, self.headers);

        rsp
    }
}
