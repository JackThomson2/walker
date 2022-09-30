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

mod v8;

mod v8_runna;

pub use db::node_functions::*;
use napi::{Result, JsFunction};
pub use request::node_functions::*;
pub use router::node_functions::*;
pub use server::node_functions::*;


#[napi]
pub fn call_ma_function(callback: JsFunction) -> Result<()> {
  let stringing = callback.coerce_to_string()?.into_utf8()?;

  println!("{}",stringing.as_str()?);

  v8_runna::do_some_cray_shit(stringing.as_str()?.to_owned());
    
  Ok(())
}