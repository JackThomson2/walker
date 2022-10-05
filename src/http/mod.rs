use std::convert::Infallible;
use std::rc::Rc;
use std::sync::Arc;

use flume::Sender;
use hyper::body::Bytes;
use hyper::server::conn::Http;
use hyper::service::{make_service_fn, service_fn, Service};
use hyper::{header, Body, Error, HeaderMap, Request, Response, Server, StatusCode};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use crate::messages::request::HttpRequest;

use self::executor::LocalExec;

mod executor;

async fn run(sender: Sender<HttpRequest>) -> Result<(), Box<dyn std::error::Error>> {
  let sender = Arc::new(sender);

  let addr = ([127, 0, 0, 1], 8081).into();

  let make_service = make_service_fn(move |_| {
    let sending = Arc::clone(&sender);

    async move {
      Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
        let (send, recv) = oneshot::channel();
        let message = HttpRequest {
          response: send,
          route: req.uri().to_string(),
        };
        sending.send(message).unwrap();

        async move {
          let res = recv.await.unwrap();

          let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::SERVER, "walker")
            .body(Body::from(res))
            .unwrap();

          Ok::<_, Error>(response)
        }
      }))
    }
  });

  let server = Server::bind(&addr).executor(LocalExec).serve(make_service);

  server.await.unwrap();

  Ok(())
}

pub fn start_http_reciever(sender: Sender<HttpRequest>) -> Result<(), Box<dyn std::error::Error>> {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("build runtime");

  rt.block_on( run(sender))
}
