use std::io;

use crate::minihttp::{HttpServiceFactory, Request, HttpService, Response};

use crate::router::read_only::get_route;
use crate::{router::store::{initialise_reader}, request::RequestBlob, Methods, oneshot::channel};

#[derive(Clone, Copy)]
struct WalkerServer;

impl WalkerServer {
    #[inline(always)]
    fn handle_function(&self, req: Request, rsp: &mut Response) {
        let method_str = req.method();
        let method = match Methods::convert_from_str(method_str) {
            Some(res) => res,
            None => {
                rsp.status_code("404", "Not Found");
                return;
            }
        };

        let result = match get_route(req.path(), method) {
            Some(res) => res,
            None => {
                rsp.status_code("404", "Not Found");
                return;
            }
        };

        let (send, rec) = channel();
        let msg_body = RequestBlob::new_with_route(req.clone(), send);

        result.call(vec![msg_body], napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking); 

        let res = match rec.recv() {
            Some(res) => res,
            None => {
                rsp.status_code("404", "Not Found");
                return;
            }
        };
        
        res.apply_to_response(rsp);
    }
}

impl HttpService for WalkerServer {
    #[inline(always)]
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        self.handle_function(req, rsp);
        
        Ok(())
    }
}

struct HttpServer;

impl HttpServiceFactory for HttpServer {
    type Service = WalkerServer;

    #[inline(always)]
    fn new_service(&self) -> Self::Service {
        WalkerServer
    }
}

#[cold]
fn configure_may() {
    may::config()
        .set_pool_capacity(10000)
        .set_stack_size(0x1000);
}

#[cold]
fn run_server(address: String) {
    let server = HttpServer.start(address).unwrap();
    server.join().unwrap();
}

#[cold]
pub fn start_server(address: String) {
    configure_may();
    initialise_reader();

    std::thread::spawn(|| {
        run_server(address);
    });
}
