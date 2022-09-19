#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate log;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod db;
mod minihttp;
mod oneshot;
mod router;
mod request;
mod types;
mod server;

pub use db::node_functions::*;
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;
