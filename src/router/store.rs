use std::{cell::UnsafeCell, collections::HashMap, mem::MaybeUninit, sync::RwLock};

use matchit::{Params, Router};
use napi::bindgen_prelude::*;

use lazy_static::lazy_static;

use crate::{types::CallBackFunction, Methods};

type ThreadSafeLookup = RwLock<Router<CallBackFunction>>;
type ReaderLookup = Router<CallBackFunction>;

lazy_static! {
  static ref GLOBAL_DATA: InternalRoutes = InternalRoutes::new_manager();
}

struct RouteCell(UnsafeCell<MaybeUninit<ReadRoutes>>);

unsafe impl Sync for RouteCell where ReadRoutes: Sync {}

static ROUTER: RouteCell = RouteCell(UnsafeCell::new(MaybeUninit::uninit()));

fn thread_to_reader(input: &ThreadSafeLookup) -> ReaderLookup {
  let reader = input.read().unwrap();
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

struct ReadRoutes {
  get: ReaderLookup,
  post: ReaderLookup,
  put: ReaderLookup,
  patch: ReaderLookup,
  delete: ReaderLookup,
}

impl ReadRoutes {
  #[inline(always)]
  fn get_for_method(&self, method: Methods) -> &ReaderLookup {
    match method {
      Methods::GET => &self.get,
      Methods::POST => &self.post,
      Methods::PUT => &self.put,
      Methods::PATCH => &self.patch,
      Methods::DELETE => &self.delete,
    }
  }
}

#[cold]
pub fn initialise_reader() {
  let new_reader = GLOBAL_DATA.as_reader_type();

  let router_ref = unsafe { &mut *ROUTER.0.get() };
  *router_ref = MaybeUninit::new(new_reader);
}

#[inline(always)]
fn get_routers() -> &'static ReadRoutes {
  unsafe { &*(*ROUTER.0.get()).as_ptr() }
}

#[cold]
pub fn add_new_route(route: &str, method: Methods, function: CallBackFunction) -> Result<()> {
  let lock = GLOBAL_DATA.get_rw_from_method(method);
  let mut writing = lock
    .write()
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))?;

  writing
    .insert(route, function)
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
}

#[inline(always)]
pub fn get_route(route: &str, method: Methods) -> Option<&CallBackFunction> {
  let checking = get_routers().get_for_method(method);
  let found = checking.at(route);

  match found {
    Ok(res) => Some(res.value),
    Err(_) => None,
  }
}

#[inline(always)]
fn params_to_map(params: &Params) -> HashMap<String, String> {
  let mut map = HashMap::with_capacity(params.len());

  for (key, value) in params.iter() {
    map.insert(key.to_string(), value.to_string());
  }

  map
}

#[inline]
pub fn get_params(route: &str, method: Methods) -> Option<HashMap<String, String>> {
  let checking = get_routers().get_for_method(method);
  let found = checking.at(route);

  match found {
    Ok(res) => Some(params_to_map(&res.params)),
    Err(_) => None,
  }
}
