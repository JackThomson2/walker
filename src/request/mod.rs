use bytes::Bytes;

use crate::minihttp::Request;
use crate::oneshot::Sender;

use self::response::JsResponse;

pub mod node_functions;
pub mod response;

#[napi]
#[derive(Clone)]
pub struct RequestBlob {
  data: Request,
  oneshot: Sender<JsResponse>,
}

impl RequestBlob {
  #[inline]
  pub fn new_with_route(data: Request, oneshot: Sender<JsResponse>) -> Self {
    Self { data, oneshot }
  }

  #[inline(always)]
  pub fn send_str(&self, string: &str) {
    let message = JsResponse::Text(Bytes::copy_from_slice(string.as_bytes()));
    unsafe { self.oneshot.send(message) }
  }
}
