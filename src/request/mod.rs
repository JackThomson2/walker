use std::{collections::HashMap, cell::RefCell, rc::Rc, sync::Arc};

use may::sync::mpsc;

pub mod node_functions;

#[napi]
#[derive(Clone, Debug)]
pub struct RequestBlob {
    data: Rc<HashMap<String, String>>,
    message: String,
    oneshot: mpsc::Sender<String>
}

impl RequestBlob {
    pub fn new_with_message(message: &str, sender:mpsc::Sender<String>) -> Self {
        Self {
            data: Rc::new(HashMap::new()),
            message: message.to_string(),
            oneshot: sender
        }
    }
}
