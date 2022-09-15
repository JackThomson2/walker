use may::sync::mpsc::Sender;
use may_minihttp::Request;

use self::response::JsResponse;

pub mod response;
pub mod node_functions;

#[napi]
#[derive(Clone, Debug)]
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
