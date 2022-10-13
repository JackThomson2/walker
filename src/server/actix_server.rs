use std::convert::Infallible;

use actix_http::{body::BoxBody, HttpService, Request, Response};
use actix_server::Server;
use actix_service::{Service, ServiceFactory};
use bytes::Bytes;
use futures::future::LocalBoxFuture;
use http::HeaderValue;
use napi::sys;
use tokio::sync::oneshot;

use crate::{
    request::{RequestBlob, unsafe_impl::store_constructor},
    router::{read_only::get_route, store::initialise_reader},
};

#[derive(Debug)]
enum Error {}

impl From<Error> for Response<BoxBody> {
    fn from(_err: Error) -> Self {
        Response::internal_server_error()
    }
}

struct ActixHttpServer {
    _hdr_srv: HeaderValue,
}

#[cold]
#[inline(never)]
fn get_failed_message() -> Result<Response<Bytes>, Infallible> {
    Ok(Response::with_body(
        http::StatusCode::NOT_FOUND,
        Bytes::new(),
    ))
}

impl Service<Request> for ActixHttpServer {
    type Response = Response<Bytes>;
    type Error = Infallible;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::always_ready!();

    #[inline(always)]
    fn call(&self, req: Request) -> Self::Future {
        Box::pin(async move {
            let result = match get_route(req.path(), req.method().clone()) {
                Some(res) => res,
                None => {
                    return get_failed_message();
                }
            };

            let (send, rec) = oneshot::channel();
            let msg_body = RequestBlob::new_with_route(req, send);

            result.call(
                msg_body,
                crate::napi::tsfn::ThreadsafeFunctionCallMode::NonBlocking,
            );

            // We'll hand back to the tokio scheduler for now as we don't expect an instant response here
            tokio::task::yield_now().await;

            match rec.await {
                Ok(res) => Ok(res.apply_to_response()),
                Err(_) => get_failed_message(),
            }
        })
    }
}

#[derive(Clone)]
struct AppFactory;

impl ServiceFactory<Request> for AppFactory {
    type Config = ();
    type Response = Response<Bytes>;
    type Error = Infallible;
    type Service = ActixHttpServer;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        Box::pin(async move {
            Ok(ActixHttpServer {
                _hdr_srv: HeaderValue::from_static("Walker"),
            })
        })
    }
}

fn run_server(address: String, workers: usize) -> std::io::Result<()> {
    actix_rt::System::new().block_on(
        Server::build()
            .backlog(1024)
            .bind("walker_server_h1", &address, || {
                HttpService::build().finish(AppFactory).tcp()
            })?
            .workers(workers)
            .run(),
    )
}

#[cold]
pub fn start_server(address: String, workers: usize, env: sys::napi_env) -> napi::Result<()> {
    initialise_reader();
    unsafe { store_constructor(env)?; }

    std::thread::spawn(move || {
        if run_server(address, workers).is_err() {
            eprintln!("Error starting server.");
        }
    });

    Ok(())
}
