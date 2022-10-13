use std::{cell::UnsafeCell, ffi::c_void, mem::MaybeUninit};

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

    Ok(())
}

#[inline(always)]
fn get_ctor() -> sys::napi_ref {
    unsafe { *(*CONSTRUCTOR.0.get()).as_ptr() }
}

#[inline(always)]
pub unsafe fn convert_to_napi(env: sys::napi_env, wrapped_value: *mut c_void) -> Option<sys::napi_value> {
    let ctor_ref = get_ctor();
    let mut ctor = std::ptr::null_mut();
    {
        let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
        match c {
            ::napi::sys::Status::napi_ok => (),
            _ => return None,
        }
    };

    let mut result = std::ptr::null_mut();
    let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
    inner.store(true, std::sync::atomic::Ordering::Relaxed);
    {
        let c = napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result);
        match c {
            ::napi::sys::Status::napi_ok => (),
            _ => return None,
        }
    };
    inner.store(false, std::sync::atomic::Ordering::Relaxed);
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
        Box::into_raw(initial_finalize),
    )));
    {
        let c = napi::sys::napi_wrap(
            env,
            result,
            wrapped_value,
            Some(napi::bindgen_prelude::raw_finalize_unchecked::<RequestBlob>),
            std::ptr::null_mut(),
            &mut object_ref,
        );
        match c {
            ::napi::sys::Status::napi_ok => Some(()),
            _ => return None,
        }
    };
    napi::bindgen_prelude::Reference::<RequestBlob>::add_ref(
        env,
        wrapped_value,
        (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Some(result)
}