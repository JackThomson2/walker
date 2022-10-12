use actix_http::{
  header::{CONTENT_TYPE, SERVER},
  Response, StatusCode,
};
use bytes::{Bytes, BytesMut, BufMut};
use http::HeaderValue;

use crate::templates::render_string_to_writer;

pub enum JsResponse {
  Text(Bytes),
  Json(Bytes),
  Raw(Bytes),
  Template(String, String, String),
  ServerError,
}

#[cold]
#[inline(never)]
fn render_internal_error() -> Response<Bytes> {
  Response::with_body(StatusCode::INTERNAL_SERVER_ERROR, Bytes::from_static(b"Internal Server Error"))
}

#[cold]
#[inline(never)]
fn render_internal_error_with_message(message: &'static [u8]) -> Response<Bytes> {
  Response::with_body(StatusCode::INTERNAL_SERVER_ERROR, Bytes::from_static(message))
}

impl JsResponse {
  #[inline(always)]
  pub fn apply_to_response(self) -> Response<Bytes> {

    let message = match &self {
      Self::Text(_) => "text/plain; charset=UTF-8",
      Self::Json(_) => "application/json; charset=UTF-8",
      Self::Raw(_) => "application/octet-stream",
      Self::Template(_,_,_) => "text/html; charset=UTF-8",
      Self::ServerError => return render_internal_error()
    };

    let mut rsp = match self {
      Self::Text(message) | Self::Json(message) => Response::with_body(StatusCode::OK, message),
      Self::Raw(data) => Response::with_body(StatusCode::OK, data),
      Self::Template(group, file, context) => {
        let buffer = BytesMut::with_capacity(2048);
        let mut writer = buffer.writer();
        
        if render_string_to_writer(&group, &file, &context, &mut writer).is_err() {
          return render_internal_error_with_message(b"Error rendering template");
        }

        let buffer = writer.into_inner();
        Response::with_body(StatusCode::OK, buffer.freeze())
      },
      _ => unreachable!()
    };

    let header = HeaderValue::from_static(message);
    let server = HeaderValue::from_static("walker");

    let hdrs = rsp.headers_mut();

    hdrs.insert(SERVER, server);
    hdrs.insert(CONTENT_TYPE, header);

    rsp
  }
}
