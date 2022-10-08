use actix_http::Request;
use async_hatch::{hatch::Hatch, Sender};
use bytes::Bytes;
use napi::{bindgen_prelude::ObjectFinalize, Env, Result};

use self::response::JsResponse;

pub mod node_functions;
pub mod response;

mod helpers;
mod writer;

#[napi(custom_finalize)]
pub struct RequestBlob {
  data: Request,
  oneshot: Sender<JsResponse, Box<Hatch<JsResponse>>>,
  sent: bool,
  body: Option<Bytes>
}

impl RequestBlob {
  #[inline]
  pub fn new_with_route(
    data: Request,
    oneshot: Sender<JsResponse, Box<Hatch<JsResponse>>>,
  ) -> Self {
    Self {
      data,
      oneshot,
      sent: false,
      body: None,
    }
  }
}

impl ObjectFinalize for RequestBlob {
  #[inline(always)]
  fn finalize(mut self, _env: Env) -> Result<()> {
    if self.sent {
      return Ok(());
    }

    self
      .oneshot
      .send(JsResponse::Text(Bytes::copy_from_slice(b"Hello world")))
      .now()
      .ok();
    Ok(())
  }
}
