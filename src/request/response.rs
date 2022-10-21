use actix_http::{
    header::{HeaderMap, HeaderName, CONTENT_TYPE, SERVER},
    Response, StatusCode,
};
use bytes::{BufMut, Bytes, BytesMut};
use http::HeaderValue;

use crate::templates::render_string_to_writer;

static WALKER_SERVER: HeaderValue = HeaderValue::from_static("walker");

static TEXT_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
static JSON_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
static RAW_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");
static HTML_HEADER_VAL: HeaderValue = HeaderValue::from_static("text/plain; charset=UTF-8");

static INTERNAL_SERVER_ERROR: Bytes = Bytes::from_static(b"Internal Server Error");


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
    EmptyString,
}

use InnerResp::*;

#[cold]
#[inline(never)]
fn render_internal_error() -> Response<Bytes> {
    Response::with_body(
        StatusCode::INTERNAL_SERVER_ERROR,
        INTERNAL_SERVER_ERROR.clone()
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
    hdrs.insert(CONTENT_TYPE, WALKER_SERVER.clone());

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
            Text(_) | EmptyString  => TEXT_HEADER_VAL.clone(),
            Json(_) => JSON_HEADER_VAL.clone(),
            Raw(_) => RAW_HEADER_VAL.clone(),
            Template(_, _, _) => HTML_HEADER_VAL.clone(),
            ServerError => return render_internal_error(),
        };

        let mut rsp = match self.inner {
            Text(message) | Json(message) => {
                Response::with_body(StatusCode::OK, message)
            }
            Raw(data) => Response::with_body(StatusCode::OK, data),
            EmptyString => Response::with_body(StatusCode::OK, Bytes::new()),
            Template(group, file, context) => {
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
