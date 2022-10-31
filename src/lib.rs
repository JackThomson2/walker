#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[cfg(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

mod db;
mod napi;
mod router;
mod request;
mod types;
mod response;
mod server;
mod templates;
mod object_pool;
mod tokio_workers;
mod extras;
mod quickjs;

pub use db::node_functions::*;
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;
pub use templates::{load_new_template, reload_group};
pub use extras::node_functions::*;
pub use quickjs::*;