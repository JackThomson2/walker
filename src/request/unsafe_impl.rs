use std::{cell::UnsafeCell, ffi::c_void, mem::MaybeUninit, sync::atomic::Ordering};

use napi::{sys, Result};

use crate::RequestBlob;

use super::helpers::make_js_error;

struct Constructor(UnsafeCell<MaybeUninit<sys::napi_ref>>);

unsafe impl Sync for Constructor {}
static CONSTRUCTOR: Constructor = Constructor(UnsafeCell::new(MaybeUninit::uninit()));

#[cold]
#[inline(never)]
pub unsafe fn store_constructor(_: sys::napi_env) -> Result<()> {
    let ctor_ref = napi::__private::get_class_constructor("RequestBlob\0")
        .ok_or_else(|| make_js_error("Error caching contructor."))?;

    let contructor_ref = &mut *CONSTRUCTOR.0.get();
    *contructor_ref = MaybeUninit::new(ctor_ref);

    let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
    inner.store(true, Ordering::Relaxed);

    Ok(())
}

#[inline(always)]
fn get_ctor() -> sys::napi_ref {
    unsafe { *(*CONSTRUCTOR.0.get()).as_ptr() }
}

#[inline(always)]
pub unsafe extern "C" fn finalize_resp(
    _: sys::napi_env,
    finalize_data: *mut c_void,
    _finalize_hint: *mut c_void,
) {
    let data = *Box::from_raw(finalize_data as *mut RequestBlob);
    drop(data)
}

#[inline(always)]
pub unsafe fn convert_to_napi(
    env: sys::napi_env,
    wrapped_value: *mut c_void,
) -> Option<sys::napi_value> {
    let ctor_ref = get_ctor();
    let mut ctor = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, ctor_ref, &mut ctor) != napi::sys::Status::napi_ok {
        return None;
    }

    let mut result = std::ptr::null_mut();
    if sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result)
        != sys::Status::napi_ok
    {
        return None;
    }

    if sys::napi_wrap(
        env,
        result,
        wrapped_value,
        Some(finalize_resp),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ) != sys::Status::napi_ok
    {
        return None;
    }
    Some(result)
}
