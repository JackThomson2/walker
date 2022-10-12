use std::ffi::c_void;

use napi::sys;

use crate::RequestBlob;

#[inline(always)]
pub unsafe fn convert_to_napi(env: sys::napi_env, blob: *mut c_void) -> Option<sys::napi_value> {
    let ctor_ref = napi::__private::get_class_constructor("RequestBlob\0")?;
    new_instance(env, blob, ctor_ref)
}

#[inline(always)]
unsafe fn new_instance(
    env: napi::sys::napi_env,
    wrapped_value: *mut std::ffi::c_void,
    ctor_ref: napi::sys::napi_ref,
) -> Option<sys::napi_value> {
    let mut ctor = std::ptr::null_mut();
    {
        let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
        match c {
            ::napi::sys::Status::napi_ok => (),
            _ => return None
        }
    };
    let mut result = std::ptr::null_mut();
    let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
    inner.store(true, std::sync::atomic::Ordering::Relaxed);
    {
        let c = napi::sys::napi_new_instance(
            env,
            ctor,
            0,
            std::ptr::null_mut(),
            &mut result,
        );
        match c {
            ::napi::sys::Status::napi_ok => (),
            _ => return None
        }
    };
    inner.store(false, std::sync::atomic::Ordering::Relaxed);
    let mut object_ref = std::ptr::null_mut();
    let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
    let finalize_callbacks_ptr = std::rc::Rc::into_raw(
        std::rc::Rc::new(std::cell::Cell::new(Box::into_raw(initial_finalize))),
    );
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
            _ => return None
        }
    };
    napi::bindgen_prelude::Reference::<
        RequestBlob,
    >::add_ref(
        env,
        wrapped_value,
        (wrapped_value, object_ref, finalize_callbacks_ptr),
    );
    Some(result)
}