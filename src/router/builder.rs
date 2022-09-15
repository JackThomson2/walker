use std::sync::Arc;

use crate::{
  request::RequestBlob, router::store::ThreadSafeLookup, Methods, server::internal::start_server,
};

use matchit::Router;
use napi::{bindgen_prelude::*, threadsafe_function::ThreadSafeCallContext};
use usync::RwLock;

use super::store::InternalRoutes;

#[napi]
pub struct ServerBuilder {
  pub(crate) get: ThreadSafeLookup,
  pub(crate) post: ThreadSafeLookup,
  pub(crate) put: ThreadSafeLookup,
  pub(crate) patch: ThreadSafeLookup,
  pub(crate) delete: ThreadSafeLookup,
}

#[napi]
impl ServerBuilder {
  #[napi]
  pub fn new_manager() -> Self {
    Self {
      get: RwLock::new(Router::new()),
      post: RwLock::new(Router::new()),
      put: RwLock::new(Router::new()),
      patch: RwLock::new(Router::new()),
      delete: RwLock::new(Router::new()),
    }
  }

  #[inline]
  fn get_rw_from_method(&self, method: Methods) -> &ThreadSafeLookup {
    match method {
      Methods::GET => &self.get,
      Methods::POST => &self.post,
      Methods::PUT => &self.put,
      Methods::PATCH => &self.patch,
      Methods::DELETE => &self.delete,
    }
  }

  #[inline]
  #[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
  /// Use this to register a new route in the server, the callback function will be called
  /// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
  /// needed to get the information from the request
  pub fn add_new_route(&self, route: String, method: Methods, callback: JsFunction) -> Result<()> {
    let tsfn = callback
      .create_threadsafe_function(1000, |ctx: ThreadSafeCallContext<Vec<RequestBlob>>| {
        Ok(ctx.value)
      })?;
    let lock = self.get_rw_from_method(method);
    let mut writing = lock.write();

    writing
      .insert(route, tsfn)
      .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
  }

  #[inline]
  #[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
  /// Adds a handler for the a GET request
  /// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
  /// needed to get the information from the request
  pub fn get(&self, route: String, callback: JsFunction) -> Result<()> {
    self.add_new_route(route, Methods::GET, callback)
  }

  #[inline]
  #[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
  /// Adds a handler for the a POST request
  /// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
  /// needed to get the information from the request
  pub fn post(&self, route: String, callback: JsFunction) -> Result<()> {
    self.add_new_route(route, Methods::POST, callback)
  }

  #[inline]
  #[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
  /// Adds a handler for the a PUT request
  /// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
  /// needed to get the information from the request
  pub fn put(&self, route: String, callback: JsFunction) -> Result<()> {
    self.add_new_route(route, Methods::PUT, callback)
  }

  #[inline]
  #[napi(ts_args_type = "route: string, method: Methods, callback: (result: RequestBlob) => void")]
  /// Adds a handler for the a PATCH request
  /// once the endpoint has been hit. The callback includes a RequestBlob which has all the methods
  /// needed to get the information from the request
  pub fn patch(&self, route: String, callback: JsFunction) -> Result<()> {
    self.add_new_route(route, Methods::PATCH, callback)
  }

  #[napi]
  pub fn start(&self, address: String) {
    let internal_routes = InternalRoutes::new_manager(self);

    start_server(address, Arc::new(internal_routes))
  }
}
