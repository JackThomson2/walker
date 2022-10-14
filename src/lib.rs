#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod db;
mod napi;
mod router;
mod request;
mod types;
mod server;
mod templates;
mod template_obj;
mod tokio_workers;
mod extras;

pub use db::node_functions::*;
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;
pub use templates::{load_new_template, reload_group};
pub use template_obj::*;
pub use extras::node_functions::*;