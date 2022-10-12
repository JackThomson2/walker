use actix_http::{
    header::{HeaderName, CONTENT_TYPE, SERVER, HeaderMap},
    Response, StatusCode,
};
use bytes::{BufMut, Bytes, BytesMut};
use http::HeaderValue;

use crate::templates::render_string_to_writer;

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
        Bytes::from_static(b"Internal Server Error"),
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
fn apply_headers(hdrs: &mut HeaderMap, message: &'static str, headers: Option<Vec<(Bytes, Bytes)>>) {
  
  let header = HeaderValue::from_static(message);
  let server = HeaderValue::from_static("walker");

  hdrs.insert(SERVER, server);
  hdrs.insert(CONTENT_TYPE, header);

  if let Some(custom_headers) = headers {
      for (key_b, val_b) in custom_headers {
          let key = match HeaderName::from_bytes(&key_b) {
              Ok(res) => res,
              Err(e) => {
                  println!("Error {:?}", e);
                  continue;
              }
          };

          let value = match HeaderValue::from_maybe_shared(val_b) {
              Ok(res) => res,
              Err(e) => {
                  println!("Error {:?}", e);
                  continue;
              }
          };

          hdrs.insert(key, value);
      }
  }

}

impl JsResponse {
    #[inline(always)]
    pub fn apply_to_response(self) -> Response<Bytes> {
        let message = match &self.inner {
            InnerResp::Text(_) => "text/plain; charset=UTF-8",
            InnerResp::Json(_) => "application/json; charset=UTF-8",
            InnerResp::Raw(_) => "application/octet-stream",
            InnerResp::Template(_, _, _) => "text/html; charset=UTF-8",
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
