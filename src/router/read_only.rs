use std::{cell::UnsafeCell, mem::MaybeUninit};

use matchit::Router;

use crate::{types::CallBackFunction, Methods};

struct RouteCell(UnsafeCell<MaybeUninit<ReadRoutes>>);

unsafe impl Sync for RouteCell where ReadRoutes: Sync {}

type ReaderLookup = Router<CallBackFunction>;
static ROUTER: RouteCell = RouteCell(UnsafeCell::new(MaybeUninit::uninit()));

pub struct ReadRoutes {
  pub get: ReaderLookup,
  pub post: ReaderLookup,
  pub put: ReaderLookup,
  pub patch: ReaderLookup,
  pub delete: ReaderLookup,
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
pub fn write_reader(new_reader: ReadRoutes) {
  let router_ref = unsafe { &mut *ROUTER.0.get() };
  *router_ref = MaybeUninit::new(new_reader);
}

#[inline(always)]
fn get_routers() -> &'static ReadRoutes {
  unsafe { &*(*ROUTER.0.get()).as_ptr() }
}

#[inline(always)]
pub fn get_route(route: &str, method: Methods) -> Option<&'static CallBackFunction> {
  let checking = get_routers().get_for_method(method);
  let found = checking.at(route);

  match found {
    Ok(res) => Some(res.value),
    Err(_) => None,
  }
}