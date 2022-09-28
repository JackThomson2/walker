
use actix_http::Request;
use async_hatch::{Sender, hatch::Hatch};

use self::response::JsResponse;

pub mod response;
pub mod node_functions;

#[napi]
pub struct RequestBlob {
    data: Request,
    oneshot: Sender<JsResponse, Box<Hatch<JsResponse>>>,
}

impl RequestBlob {
    #[inline]
    pub fn new_with_route(data: Request, oneshot: Sender<JsResponse, Box<Hatch<JsResponse>>>) -> Self {
        Self {
            data,
            oneshot
        }
    }
}
