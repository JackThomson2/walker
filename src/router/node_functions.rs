use napi::{bindgen_prelude::*, threadsafe_function::ThreadSafeCallContext};

use crate::{request::RequestBlob, router::store::add_new_route};

#[napi]
pub fn new_route(route: String, callback: JsFunction) -> Result<()> {
  let tsfn =
    callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Vec<RequestBlob>>| {
      Ok(ctx.value)
    })?;

  add_new_route(&route, tsfn)?;
  
  Ok(())
}
