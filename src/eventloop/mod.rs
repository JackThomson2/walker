use std::{cell::UnsafeCell, ptr, sync::Arc, time::Instant};

use napi::NapiRaw;

use napi::{
  bindgen_prelude::FromNapiValue, sys::napi_value, CallContext, ContextlessResult, Env, JsFunction,
  JsObject, JsString, JsUndefined, Result, Value,
};

struct Inner {
  value: napi_value,
  reffing: napi::sys::napi_ref,
}

impl Inner {
  fn new(value: napi_value, reffing: napi::sys::napi_ref) -> Self {
    Self { value, reffing }
  }
}

struct CallbackCell(UnsafeCell<Option<Inner>>);

static CallingBackFunc: CallbackCell = CallbackCell(UnsafeCell::new(None));

unsafe impl Sync for CallbackCell {}

#[js_function(1)]
fn registerFunction(ctx: CallContext) -> Result<JsUndefined> {
  let mut raw_ref = ptr::null_mut();
  let input_number = ctx.args[0];

  let _res =
    unsafe { napi::sys::napi_create_reference(ctx.env.raw(), input_number, 2, &mut raw_ref) };

  let callback_ref = unsafe { &mut *CallingBackFunc.0.get() };
  *callback_ref = Some(Inner::new(input_number, raw_ref));

  call_anon_function(&ctx.env)?;

  ctx.env.get_undefined()
}

fn call_anon_function(env: &Env) -> Result<()> {
  let calling = unsafe { (*CallingBackFunc.0.get()).as_ref().unwrap() };

  let mut raw_ref = ptr::null_mut();
  let _ = unsafe { napi::sys::napi_get_reference_value(env.raw(), calling.reffing, &mut raw_ref) };
  let undef = unsafe { env.get_undefined()?.raw() };

  let mut resulting = ptr::null_mut();
  unsafe {
    napi::sys::napi_call_function(env.raw(), undef, raw_ref, 0, ptr::null_mut(), &mut resulting);
  }
  Ok(())
}

#[contextless_function]
pub fn get_event_loop(env: Env) -> ContextlessResult<JsUndefined> {
  call_anon_function(&env)?;

  env.get_undefined().map(Some)
}

pub fn register_js(exports: &mut JsObject) -> Result<()> {
  exports.create_named_method("eventLoop", get_event_loop)?;
  exports.create_named_method("registerConst", registerFunction)?;

  Ok(())
}
