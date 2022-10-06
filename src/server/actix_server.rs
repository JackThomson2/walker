use actix_http::{body::BoxBody, HttpService, Request, Response};
use actix_server::Server;
use actix_service::{Service, ServiceFactory};
use async_hatch::oneshot;
use bytes::Bytes;
use futures::future::LocalBoxFuture;
use http::HeaderValue;

use crate::{
  request::RequestBlob,
  router::{read_only::get_route, store::initialise_reader},
  Methods,
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
fn get_failed_message() -> Result<Response<Bytes>, Error> {
  Ok(Response::with_body(
    http::StatusCode::NOT_FOUND,
    Bytes::new(),
  ))
}

impl Service<Request> for ActixHttpServer {
  type Response = Response<Bytes>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  actix_service::always_ready!();

  #[inline(always)]
  fn call(&self, req: Request) -> Self::Future {
    Box::pin(async move {
      let method = match Methods::convert_from_str(req.method().as_str()) {
        Some(res) => res,
        None => {
          return get_failed_message();
        }
      };

      let result = match get_route(req.path(), method) {
        Some(res) => res,
        None => {
          return get_failed_message();
        }
      };

      let (send, rec) = oneshot();
      let msg_body = RequestBlob::new_with_route(req, send);

      result.call(
        msg_body,
        crate::napi::tsfn::ThreadsafeFunctionCallMode::NonBlocking,
      );

      // We'll hand back to the tokio scheduler for now as we don't expect an instant response here
      tokio::task::yield_now().await;

      match rec.close_on_receive(true).receive().await {
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
  type Error = Error;
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

fn run_server(address: String) -> std::io::Result<()> {
  actix_rt::System::new().block_on(
    Server::build()
      .bind("walker_server_h1", &address, || {
        HttpService::build().finish(AppFactory).tcp()
      })?
      .run(),
  )
}

#[cold]
pub fn start_server(address: String) {
  initialise_reader();

  std::thread::spawn(move || {
    run_server(address).unwrap();
  });
}
