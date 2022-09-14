use may::sync::mpsc::Sender;
use may_minihttp::Request;

pub mod node_functions;

#[napi]
#[derive(Clone, Debug)]
pub struct RequestBlob {
    data: Request,
    oneshot: Sender<String>
}

impl RequestBlob {
    pub fn new_with_route(data: Request, oneshot: Sender<String>) -> Self {
        Self {
            data,
            oneshot
        }
    }
}
