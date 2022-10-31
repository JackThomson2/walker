use std::collections::HashMap;

use quick_js::{Context, JsValue};
use tokio::sync::oneshot::Sender;
use flume::Receiver;

use crate::response::JsResponse;

pub type WorkerMessage = ((String, String), Sender<JsResponse>);

pub type WorkerSender = flume::Sender<WorkerMessage>;
pub type WorkerRecv = Receiver<WorkerMessage>;

pub fn run_worker(listener: WorkerRecv) {
    let context = Context::new().unwrap();
    let global = make_js_global_req_obj();
    context.set_global("request", global).unwrap();

    context.eval(r#"
        const recv = (request) => {
            return `Result is ${request.url} and ${request.method}`;
        }
    "#).unwrap();

    loop {
        let (req, sender) = listener.recv().unwrap();

        let to_eval = format!("request.url = '{}'; request.method = '{}';", req.0, req.1);
        context.eval(&to_eval).unwrap();

        let res = context.eval_as::<String>("recv(request)");

        match res {
            Ok(res) => {
                let response = JsResponse::simple_str(res);
                sender.send(response).ok();
            },
            Err(_) => {
                let response = JsResponse::error();
                sender.send(response).ok();
            }
        }
    }
}

fn make_js_global_req_obj() -> JsValue {
    // Build a basic request object with a url and method
    
    let mut req_obj = HashMap::<String, JsValue>::new();

    req_obj.insert("url".to_string(), JsValue::from("http://localhost:3000"));
    req_obj.insert("method".to_string(), JsValue::from("GET"));

    JsValue::from(req_obj)
}

pub fn make_workers(reciever: WorkerRecv, thread_count: usize) {
    for _ in 0..thread_count {
        let reciever = reciever.clone();
        std::thread::spawn(move || {
            run_worker(reciever);
        });
    }
}