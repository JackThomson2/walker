use matchit::Router;
use napi::sys::napi_ref;

pub mod node_functions;
pub mod read_only;
pub mod store;
mod hackery;


pub type ReaderLookup = Router<napi_ref>;