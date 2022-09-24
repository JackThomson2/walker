use std::ptr;

use napi::{JsFunction, sys::napi_ref, NapiRaw, Env};

#[cold]
pub unsafe fn convert_js_func_to_shared_ref(env: &Env, function: JsFunction) -> napi_ref {
    let mut raw_ref = ptr::null_mut();
    let input_function = (&function).raw();

    let _res = napi::sys::napi_create_reference(env.raw(), input_function, 1, &mut raw_ref);

    raw_ref
}