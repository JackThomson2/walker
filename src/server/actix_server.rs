use actix_http::{
  body::BoxBody,
  header::{CONTENT_TYPE, SERVER},
  HttpService, Request, Response, StatusCode,
};
use actix_server::Server;
use actix_service::{Service, ServiceFactory};
use async_hatch::oneshot;
use bytes::Bytes;
use futures::future::{ok, LocalBoxFuture};
use http::HeaderValue;

use crate::{
  request::{response::JsResponse, RequestBlob},
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
  hdr_srv: HeaderValue,
}

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
      let method = match Methods::convert_from_str("GET") {
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
        vec![msg_body],
        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
      );

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
        hdr_srv: HeaderValue::from_static("Walker"),
      })
    })
  }
}

fn run_server(address: String) -> std::io::Result<()> {
  actix_rt::System::new().block_on(
    Server::build()
      .bind("walker_server_h1", &address, || {
        HttpService::build().h1(AppFactory).tcp()
      })?
      .bind("walker_server_h2", &address, || {
        HttpService::build().h2(AppFactory).tcp()
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
