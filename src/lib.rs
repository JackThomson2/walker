#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod db;
mod oneshot;
mod router;
mod request;
mod types;
mod server;

pub use db::node_functions::*;
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;
