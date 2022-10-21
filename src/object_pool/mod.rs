use std::{ffi::c_void, sync::atomic::Ordering};

use napi::{sys, Result};
use parking_lot::Mutex;

use crate::request::{helpers::make_js_error, RequestBlob};

pub struct StoredPair(pub (Box<RequestBlob>, sys::napi_ref));

unsafe impl Send for StoredPair {}
unsafe impl Sync for StoredPair {}

static POOL: Mutex<Vec<StoredPair>> = Mutex::new(vec![]);

pub fn get_stored_chunk(count: usize) -> Vec<StoredPair> {
    let mut locked = POOL.lock();
    let split_point = locked.len() - count;

    locked.split_off(split_point)
}

unsafe fn get_obj_constructor() -> Result<sys::napi_ref> {
    let ctor_ref = napi::__private::get_class_constructor("RequestBlob\0")
        .ok_or_else(|| make_js_error("Error caching contructor."))?;

    let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
    inner.store(true, Ordering::Relaxed);

    Ok(ctor_ref)
}

pub unsafe fn build_up_pool(env: sys::napi_env, pool_size: usize) -> Result<()> {
    let ctor_ref = get_obj_constructor()?;
    let mut ctor = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, ctor_ref, &mut ctor) != napi::sys::Status::napi_ok {
        return Err(make_js_error("Error getting constructor."));
    }

    let mut locked_pool = POOL.lock();
    locked_pool.reserve(pool_size);

    println!("Pooling objects");

    for _ in 0..pool_size {
        let mut result = std::ptr::null_mut();
        if sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result)
            != sys::Status::napi_ok
        {
            return Err(make_js_error("Error creating a new instance."));
        }

        let mut reffering = std::ptr::null_mut();
        if sys::napi_create_reference(env, result, 1, &mut reffering) != sys::Status::napi_ok {
            return Err(make_js_error("Error creating the reference."));
        }

        let mut found_obj = std::ptr::null_mut();
        if sys::napi_get_reference_value(env, reffering, &mut found_obj) != sys::Status::napi_ok {
            return Err(make_js_error("Error doing the deref."));
        }

        let native_object = RequestBlob::new_empty_with_js();
        let raw_obj = Box::into_raw(native_object);

        let _result = sys::napi_wrap(
            env,
            found_obj,
            raw_obj as *mut c_void,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        let recovered = Box::from_raw(raw_obj);
        locked_pool.push(StoredPair((recovered, reffering)));
    }

    Ok(())
}
