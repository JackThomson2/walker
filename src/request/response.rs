use actix_http::{
  header::{CONTENT_TYPE, SERVER},
  Response, StatusCode,
};
use bytes::Bytes;
use http::HeaderValue;

pub enum JsResponse {
  Text(Bytes),
  Json(Bytes),
  Raw(Bytes),
}

impl JsResponse {
  #[inline(always)]
  pub fn apply_to_response(self) -> Response<Bytes> {
    let message = match &self {
      Self::Text(_) => "text/plain; charset=UTF-8",
      Self::Json(_) => "application/json; charset=UTF-8",
      Self::Raw(_) => "application/octet-stream",
    };

    let mut rsp = match self {
      Self::Text(message) | Self::Json(message) => Response::with_body(StatusCode::OK, message),
      Self::Raw(data) => Response::with_body(StatusCode::OK, data)
    };

    let header = HeaderValue::from_static(message);
    let server = HeaderValue::from_static("walker");

    let hdrs = rsp.headers_mut();

    hdrs.insert(SERVER, server);
    hdrs.insert(CONTENT_TYPE, header);

    rsp
  }
}
