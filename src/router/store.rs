use std::{sync::{RwLock, atomic::AtomicBool, Mutex}, cell::UnsafeCell, mem::MaybeUninit};

use matchit::Router;
use napi::{bindgen_prelude::*, sys::napi_ref};

use crate::{types::CallBackFunction, Methods};

use super::{read_only::{write_reader, ReadRoutes}, ReaderLookup};

type ThreadSafeLookup = RwLock<ReaderLookup>;

struct InternalRouter(UnsafeCell<MaybeUninit<InternalRoutes>>);

unsafe impl Sync for InternalRouter {}

static GLOBAL_DATA: InternalRouter = InternalRouter(UnsafeCell::new(MaybeUninit::uninit()));
static INITED: Mutex<bool> = Mutex::new(false);

#[cold]
fn init_globals() {
  let manager = InternalRoutes::new_manager();
  let global_store = unsafe { &mut *GLOBAL_DATA.0.get() };
  *global_store = MaybeUninit::new(manager);
}

fn get_global() -> &'static InternalRoutes {
  let mut initialised = INITED.lock().unwrap();

  if !*initialised {
    init_globals();
    *initialised = true;
  }

  drop(initialised);

  unsafe { &*(*GLOBAL_DATA.0.get()).as_ptr() }
} 

pub fn thread_to_reader(input: &ThreadSafeLookup) -> ReaderLookup {
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

#[cold]
pub fn initialise_reader() {
  let globo_data = get_global();
  let new_reader = globo_data.as_reader_type();

  write_reader(new_reader);
}

#[cold]
pub fn add_new_route(route: &str, method: Methods, function: napi_ref) -> Result<()> {
  let globo_data = get_global();
  let lock = globo_data.get_rw_from_method(method);
  let mut writing = lock
    .write()
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))?;

  writing
    .insert(route, function)
    .map_err(|_| Error::new(Status::GenericFailure, "Error inserting route".to_string()))
}
