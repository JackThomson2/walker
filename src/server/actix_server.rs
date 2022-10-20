use std::{cell::UnsafeCell, convert::Infallible, rc::Rc};

use actix_http::{body::BoxBody, HttpService, Request, Response};
use actix_server::Server;
use actix_service::{Service, ServiceFactory};
use bytes::Bytes;
use futures::future::LocalBoxFuture;
use http::HeaderValue;
use napi::sys;
use tokio::sync::oneshot;

use crate::{
    extras::scheduler::{pin_js_thread, try_pin_priority},
    request::{
        request_pool::{build_up_pool, get_stored_chunk, StoredPair},
        unsafe_impl::store_constructor,
    },
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
    object_pool: Rc<UnsafeCell<Vec<StoredPair>>>,
}

#[cold]
#[inline(never)]
fn get_failed_message() -> Result<Response<Bytes>, Infallible> {
    Ok(Response::with_body(
        http::StatusCode::NOT_FOUND,
        Bytes::new(),
    ))
}

impl ActixHttpServer {
    #[inline(always)]
    fn get_mut_from_unsafe<'a>(
        unsafe_cell: &'a UnsafeCell<Vec<StoredPair>>,
    ) -> &'a mut Vec<StoredPair> {
        unsafe { &mut *unsafe_cell.get() }
    }
}

impl Service<Request> for ActixHttpServer {
    type Response = Response<Bytes>;
    type Error = Infallible;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::always_ready!();

    #[inline(always)]
    fn call(&self, req: Request) -> Self::Future {
        let vec_ref = self.object_pool.clone();

        Box::pin(async move {
            let result = match get_route(req.path(), req.method().clone()) {
                Some(res) => res,
                None => {
                    return get_failed_message();
                }
            };

            let to_add_back = Self::get_mut_from_unsafe(&vec_ref);
            let mut to_use = to_add_back.pop().unwrap();

            let (send, rec) = oneshot::channel();
            to_use.0.0.store_self_data(req, send);

            result.call(
                to_use.0 .1,
                crate::napi::tsfn::ThreadsafeFunctionCallMode::NonBlocking,
            );

            // We'll hand back to the tokio scheduler for now as we don't expect an instant response here
            // tokio::task::yield_now().await;

            let result = match rec.await {
                Ok(res) => Ok(res.apply_to_response()),
                Err(_) => get_failed_message(),
            };

            if to_add_back.len() == to_add_back.capacity() {
                unsafe { std::hint::unreachable_unchecked() }
            }

            to_add_back.push(to_use);

            result
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
        // Set non priority here..
        try_pin_priority();

        Box::pin(async move {
            Ok(ActixHttpServer {
                _hdr_srv: HeaderValue::from_static("Walker"),
                object_pool: unsafe { Rc::new(UnsafeCell::new(get_stored_chunk(5_000))) },
            })
        })
    }
}

fn run_server(address: String, workers: usize) -> std::io::Result<()> {
    // Lets set net reciever priority here
    try_pin_priority();

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
    unsafe {
        store_constructor(env)?;
    }

    unsafe {
        build_up_pool(env);
    }

    // Lets set js priority here
    pin_js_thread();

    std::thread::spawn(move || {
        if run_server(address, workers).is_err() {
            eprintln!("Error starting server.");
        }
    });

    Ok(())
}
