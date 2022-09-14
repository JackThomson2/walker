use std::{io, fmt::Write};

use may::{sync::mpsc};
use may_minihttp::{HttpServiceFactory, Request, HttpService, Response};

use crate::{router::store::get_route, request::RequestBlob};

struct WalkerServer;

impl WalkerServer {
    fn handle_function(&self, req: &Request, rsp: &mut Response) {
        let result = match get_route(req.path()) {
            Some(res) => res,
            None => {
                rsp.status_code("404", "Not Found");
                return;
            }
        };

        let (send, rec) = mpsc::channel();
        let msg_body = RequestBlob::new_with_route(req.clone(), send);

        result.call(vec![msg_body], napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking); 

        let res = match rec.recv() {
            Ok(res) => res,
            Err(_) => {
                rsp.status_code("404", "Not Found");
                return;
            }
        };

        let bytes = rsp.body_mut();
        if bytes.write_str(&res).is_err() {
            println!("Error writing message...");
        };
    }
}

impl HttpService for WalkerServer {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        self.handle_function(&req, rsp);
        
        Ok(())
    }
}

struct HttpServer;

impl HttpServiceFactory for HttpServer {
    type Service = WalkerServer;

    fn new_service(&self) -> Self::Service {
        WalkerServer
    }
}

#[inline]
fn configure_may() {
    may::config()
        .set_pool_capacity(10000)
        .set_stack_size(0x1000);
}

#[inline]
fn run_server(address: String) {
    let server = HttpServer.start(address).unwrap();
    server.join().unwrap();
}

#[inline]
pub fn start_server(address: String) {
    configure_may();

    std::thread::spawn(|| {
        run_server(address);
    });
}
