use std::mem::MaybeUninit;

use actix_http::Request;
use bytes::Bytes;
use tokio::sync::oneshot::Sender;

use self::response::JsResponse;

pub mod helpers;
pub mod node_functions;
pub mod response;
pub mod writer;

#[napi]
pub struct RequestBlob {
  data: Request,
  oneshot: MaybeUninit<Sender<JsResponse>>,
  sent: bool,
  body: Option<Bytes>
}

impl RequestBlob {
  #[inline]
  pub fn new_with_route(
    data: Request,
    sender: Sender<JsResponse>,
  ) -> Self {
    let oneshot = MaybeUninit::new(sender);

    Self {
      data,
      oneshot,
      sent: false,
      body: None,
    }
  }
}
