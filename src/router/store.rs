use matchit::Router;
use napi::bindgen_prelude::*;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::{types::CallBackFunction, Methods};

use super::read_only::{write_reader, ReadRoutes};

type ReaderLookup = Router<CallBackFunction>;
type ThreadSafeLookup = RwLock<Router<CallBackFunction>>;

lazy_static! {
  static ref GLOBAL_DATA: InternalRoutes = InternalRoutes::new_manager();
}

pub fn thread_to_reader(input: &ThreadSafeLookup) -> ReaderLookup {
  let reader = input.read();
  reader.clone()
}

struct InternalRoutes {
  get: ThreadSafeLookup,
  post: ThreadSafeLookup,
  put: ThreadSafeLookup,
  patch: ThreadSafeLookup,
  delete: ThreadSafeLookup,
}

impl InternalRoutes {
  #[cold]
  fn new_manager() -> Self {
    Self {
      get: RwLock::new(Router::new()),
      post: RwLock::new(Router::new()),
      put: RwLock::new(Router::new()),
      patch: RwLock::new(Router::new()),
      delete: RwLock::new(Router::new()),
    }
  }

  #[cold]
  fn get_rw_from_method(&self, method: Methods) -> &ThreadSafeLookup {
    match method {
      Methods::GET => &self.get,
      Methods::POST => &self.post,
      Methods::PUT => &self.put,
      Methods::PATCH => &self.patch,
      Methods::DELETE => &self.delete,
    }
  }

  #[cold]
  fn as_reader_type(&self) -> ReadRoutes {
    ReadRoutes {
      get: thread_to_reader(&self.get),
      post: thread_to_reader(&self.post),
      put: thread_to_reader(&self.put),
      patch: thread_to_reader(&self.patch),
      delete: thread_to_reader(&self.delete),
    }
  }
}

#[cold]
pub fn initialise_reader() {
  let new_reader = GLOBAL_DATA.as_reader_type();

  write_reader(new_reader);
}

#[cold]
pub fn add_new_route(route: &str, method: Methods, function: CallBackFunction) -> Result<()> {
  let lock = GLOBAL_DATA.get_rw_from_method(method);
  let mut writing = lock
    .write();

  writing
    .insert(route, function)
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
}
