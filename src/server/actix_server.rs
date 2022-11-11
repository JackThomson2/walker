use std::{cell::UnsafeCell, convert::Infallible, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use actix_http::{HttpService, Request, Response};
use actix_server::Server;
use actix_service::{Service, ServiceFactory};
use bytes::Bytes;
use futures::future::LocalBoxFuture;
use napi::sys;
use tokio::sync::oneshot;

use crate::{
    extras::scheduler::{pin_js_thread, try_pin_priority, reset_thread_affinity},
    object_pool::{build_up_pool, StoredPair, get_pool_for_threads},
    router::{read_only::get_route, store::initialise_reader}, request::helpers::make_js_error,
};

use super::{
    config::ServerConfig,
    helpers::{get_failed_message, get_post_body}, shutdown::{attach_server_handle, try_own_start},
};

struct ActixHttpServer {
    object_pool: Rc<UnsafeCell<Vec<Vec<StoredPair>>>>,
    idx: UnsafeCell<usize>
}

impl ActixHttpServer {
    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn get_mut_from_unsafe(unsafe_cell: &UnsafeCell<Vec<Vec<StoredPair>>>) -> &mut Vec<Vec<StoredPair>> {
        unsafe { &mut *unsafe_cell.get() }
    }

    #[inline(never)]
    #[cold]
    async fn backoff_get_object(items: &mut Vec<StoredPair>) -> StoredPair {
        loop {
            tokio::task::yield_now().await;

            if let Some(retrieved) = items.pop() {
                return retrieved;
            }
        }
    }

    #[inline(always)]
    fn get_next_idx(&self) -> usize {
        let position = unsafe { &mut *self.idx.get() };
        let val = *position;

        *position += 1;

        val
    }
}

impl Service<Request> for ActixHttpServer {
    type Response = Response<Bytes>;
    type Error = Infallible;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::always_ready!();

    #[inline(always)]
    fn call(&self, mut req: Request) -> Self::Future {
        let vec_ref = self.object_pool.clone();
        let offset = self.get_next_idx();

        Box::pin(async move {
            let router = match get_route(req.path(), req.method().clone()) {
                Some(res) => res,
                None => {
                    return get_failed_message();
                }
            };

            let router = unsafe { router.get_unchecked(offset % router.len()) };

            let mut body = None;

            if req.method() == http::Method::POST {
                body = match get_post_body(req.payload()).await {
                    Ok(body) => Some(body),
                    Err(_) => {
                        return get_failed_message();
                    }
                };
            }

            let object_reference = Self::get_mut_from_unsafe(&vec_ref);

            let mut js_obj = unsafe {
                let reference = object_reference.get_unchecked_mut(router.threads_id);

                match reference.pop() {
                    Some(res) => res,
                    None => Self::backoff_get_object(reference).await,
                }
            };

            let (send, rec) = oneshot::channel();
            
            js_obj.0 .0.store_self_data(req, send, body);

            router.function.call(
                js_obj.0 .1,
                crate::napi::tsfn::ThreadsafeFunctionCallMode::NonBlocking,
            );

            let result = match rec.await {
                Ok(res) => Ok(res.apply_to_response()),
                Err(_) => get_failed_message(),
            };

            unsafe {
                let reference = object_reference.get_unchecked_mut(router.threads_id);
                reference.push(js_obj);
            }

            result
        })
    }
}

#[derive(Clone)]
struct AppFactory(usize);

static IDX_OFFSETTER: AtomicUsize = AtomicUsize::new(0);

impl ServiceFactory<Request> for AppFactory {
    type Config = ();
    type Response = Response<Bytes>;
    type Error = Infallible;
    type Service = ActixHttpServer;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        try_pin_priority();

        let chunk_size = self.0;
        let object_pool = Rc::new(UnsafeCell::new(get_pool_for_threads(chunk_size).unwrap()));

        Box::pin(async move {
            Ok(ActixHttpServer {
                object_pool,
                idx: UnsafeCell::new(IDX_OFFSETTER.fetch_add(1, Ordering::SeqCst)),
            })
        })
    }
}

async fn create_sever(config: ServerConfig) -> std::io::Result<()> {
    let pool_size = config.get_pool_per_worker();

    let srv = Server::build()
        .backlog(config.get_backlog_size())
        .bind("walker_server_h1", &config.url, move || {
            HttpService::build().finish(AppFactory(pool_size as usize)).tcp()
        })?
        .workers(config.get_worker_thread() as usize)
        .run();

    attach_server_handle(srv.handle());

    srv.await
}

async fn create_tls_server(config: ServerConfig) -> std::io::Result<()> {
    let pool_size = config.get_pool_per_worker();
    let certs = super::tls::load_tls_certs(&config).unwrap();

    let srv = Server::build()
        .backlog(config.get_backlog_size())
        .bind("walker_server_h1", &config.url, move || {
            HttpService::build().finish(AppFactory(pool_size as usize)).rustls(certs.clone())
        })?
        .workers(config.get_worker_thread() as usize)
        .run();

    attach_server_handle(srv.handle());

    srv.await
}


fn run_server(config: ServerConfig) -> std::io::Result<()> {
    // Lets set net reciever priority here
    try_pin_priority();

    if config.get_tls() {
        actix_rt::System::new().block_on(create_tls_server(config))
    } else {
        actix_rt::System::new().block_on(create_sever(config))
    }
}

#[cold]
pub fn start_server(config: ServerConfig, env: sys::napi_env) -> napi::Result<()> {
    if !try_own_start() {
        return Err(make_js_error("Server already started"));
    }
    
    reset_thread_affinity();
    initialise_reader();
    unsafe { build_up_pool(env, config.get_pool_size())?; }

    // Lets set js priority here
    pin_js_thread();

    std::thread::spawn(move || {
        if run_server(config).is_err() {
            eprintln!("Error starting server.");
        }
    });

    Ok(())
}
