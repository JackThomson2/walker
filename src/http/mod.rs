use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use http_body_util::Full;
use hyper::body::{Bytes, HttpBody};
use hyper::http::HeaderValue;
use hyper::server::conn::Http;
use hyper::service::{service_fn, Service};
use hyper::{Error, HeaderMap, Request, Response};
use tokio::net::TcpListener;

use crate::v8::State;

use self::executor::LocalExec;

mod executor;

struct Body {
  // Our Body type is !Send and !Sync:
  _marker: PhantomData<*const ()>,
  data: Option<Bytes>,
}

impl From<String> for Body {
  fn from(a: String) -> Self {
    Body {
      _marker: PhantomData,
      data: Some(a.into()),
    }
  }
}

impl HttpBody for Body {
  type Data = Bytes;
  type Error = Error;

  fn poll_data(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
  ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
    Poll::Ready(self.get_mut().data.take().map(Ok))
  }

  fn poll_trailers(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
  ) -> Poll<Result<Option<HeaderMap<HeaderValue>>, Self::Error>> {
    Poll::Ready(Ok(None))
  }
}

struct Svc<'s, 'i> {
  state: Rc<RefCell<State<'s, 'i>>>,
}

impl<'s, 'i> Service<Request<hyper::Body>> for Svc<'s, 'i> {
  type Response = Response<Body>;
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn call(&mut self, req: Request<hyper::Body>) -> Self::Future {
    let found = match self.state.borrow_mut().find_and_run_route(req.uri().path()) {
      Some(res) => res,
      None => "Not found".to_owned(),
    };

    let response = Response::new(Body::from(found));
    Box::pin(async { Ok::<_, Infallible>(response) })
  }

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }
}

pub fn start_http_reciever(
  listener: std::net::TcpListener,
  source: String,
) -> Result<(), Box<dyn std::error::Error>> {
  let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("build runtime");

  rt.block_on(async {
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let mut scope = v8::HandleScope::new(&mut isolate);
    let source = v8::String::new(&mut scope, &source).unwrap();

    let mut state_machine = State::new(&mut scope, source);
    state_machine.prepare();

    let states = Rc::new(RefCell::new(state_machine));

    let tokio_listener = TcpListener::from_std(listener)?;

    loop {
      let (stream, _) = tokio_listener.accept().await?;

      let state = states.clone();

      async move {
        if let Err(_) = Http::new()
          .with_executor(LocalExec)
          .serve_connection(stream, Svc { state })
          .await
        {}
      }.await;
    }
  })
}
