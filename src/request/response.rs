use actix_http::{
  header::{CONTENT_TYPE, SERVER},
  Response, StatusCode,
};
use bytes::{Bytes, BytesMut};
use http::HeaderValue;

use crate::templates::render_string_to_writer;

use super::writer::Writer;

pub enum JsResponse {
  Text(Bytes),
  Json(Bytes),
  Raw(Bytes),
  Template(String, String, String),
}

impl JsResponse {
  #[inline(always)]
  pub fn apply_to_response(self) -> Response<Bytes> {
    let message = match &self {
      Self::Text(_) => "text/plain; charset=UTF-8",
      Self::Json(_) => "application/json; charset=UTF-8",
      Self::Raw(_) => "application/octet-stream",
      Self::Template(_,_,_) => "text/html; charset=UTF-8"
    };

    let mut rsp = match self {
      Self::Text(message) | Self::Json(message) => Response::with_body(StatusCode::OK, message),
      Self::Raw(data) => Response::with_body(StatusCode::OK, data),
      Self::Template(group, file, context) => {
        let mut buffer = BytesMut::with_capacity(2048);
        render_string_to_writer(&group, &file, &context, &mut Writer(&mut buffer)).unwrap();
        Response::with_body(StatusCode::OK, buffer.freeze())
      }
    };

    let header = HeaderValue::from_static(message);
    let server = HeaderValue::from_static("walker");

    let hdrs = rsp.headers_mut();

    hdrs.insert(SERVER, server);
    hdrs.insert(CONTENT_TYPE, header);

    rsp
  }
}
