
use may_minihttp::Request;

use crate::oneshot::Sender;

use self::response::JsResponse;

pub mod response;
pub mod node_functions;

#[napi]
#[derive(Clone)]
pub struct RequestBlob {
    data: Request,
    oneshot: Sender<JsResponse>,
}

impl RequestBlob {
    pub fn new_with_route(data: Request, oneshot: Sender<JsResponse>) -> Self {
        Self {
            data,
            oneshot
        }
    }
}
