pub mod messagebus;
pub mod request;

use std::{cell::UnsafeCell, ptr, sync::Arc, time::Instant};

use napi::NapiRaw;

use napi::{
  bindgen_prelude::FromNapiValue, sys::napi_value, CallContext, ContextlessResult, Env, JsFunction,
  JsObject, JsString, JsUndefined, Result, Value,
};

use crate::request::RequestBlob;


#[js_function(2)]
fn registerFunction(ctx: CallContext) -> Result<JsUndefined> {
  let wrapped_obj: JsObject = ctx.get::<JsObject>(0)?;
  let inner_function: &mut RequestBlob = ctx.env.unwrap(&wrapped_obj)?;

  let input_string = ctx.get::<JsString>(1)?.into_utf8()?;
  inner_function.send_str(input_string.as_str()?);

  ctx.env.get_undefined()
}

#[inline]
#[contextless_function]
pub fn get_event_loop(env: Env) -> ContextlessResult<JsUndefined> {
  let result = messagebus::get_reader();

  while let Ok(found) = result.recv() {
    unsafe { found.call_method(&env)?; }
  }

  env.get_undefined().map(Some)
}

pub fn register_js(exports: &mut JsObject) -> Result<()> {
  exports.create_named_method("eventLoop", get_event_loop)?;
  exports.create_named_method("registerConst", registerFunction)?;

  Ok(())
}
