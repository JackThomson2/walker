#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate log;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod db;
mod eventloop;
mod minihttp;
mod oneshot;
mod router;
mod request;
mod types;
mod server;

pub use db::node_functions::*;
pub use eventloop::*;
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;

use napi::*;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    eventloop::register_js(&mut exports)?;

    Ok(())
}