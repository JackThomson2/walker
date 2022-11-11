use std::{ffi::c_void, sync::atomic::Ordering};

use napi::{sys, Result};
use parking_lot::{Mutex, RwLock};

use crate::request::{helpers::make_js_error, RequestBlob};

pub struct StoredPair(pub (Box<RequestBlob>, sys::napi_ref));

unsafe impl Send for StoredPair {}
unsafe impl Sync for StoredPair {}

static WORKER_POOL: RwLock<Vec<Mutex<Vec<StoredPair>>>> = RwLock::new(vec![]);

pub unsafe fn build_pool_for_id(env: sys::napi_env, pool_size: usize, thread_id: usize, build_if_present: bool) -> Result<()> {
    let mut pool_list = WORKER_POOL.write();

    if pool_list.len() >= thread_id {
        pool_list.resize_with(thread_id + 1, Default::default);
    }

    let found = pool_list.get_mut(thread_id).ok_or_else(|| make_js_error("Error building pool"))?;
    let mut pool = found.lock();

    if !build_if_present && !pool.is_empty() {
        return Ok(())
    }

    build_pool_into_vec(env, pool_size, &mut pool)
}

pub fn get_pool_for_threads(count: usize) -> Result<Vec<Vec<StoredPair>>> {
    let locked = WORKER_POOL.read();
    let mut result = Vec::with_capacity(locked.len());

    for thread in locked.iter() {
        let mut obj_list = thread.lock();
        if obj_list.len() < count {
            println!("The object length is {} and count is {}", obj_list.len(), count);
            return Err(make_js_error("We don't have enough objects provisioned."))
        }

        let split_point = obj_list.len() - count;
        let split = obj_list.split_off(split_point);

        result.push(split);
    } 

    Ok(result)
}

unsafe fn get_obj_constructor() -> Result<sys::napi_ref> {
    let ctor_ref = napi::__private::get_class_constructor("RequestBlob\0")
        .ok_or_else(|| make_js_error("Error caching contructor."))?;

    let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
    inner.store(true, Ordering::Relaxed);

    Ok(ctor_ref)
}

unsafe fn build_pool_into_vec(env: sys::napi_env, pool_size: usize, pool: &mut Vec<StoredPair>) -> Result<()> {
    pool.reserve(pool_size);

    let ctor_ref = get_obj_constructor()?;
    let mut ctor = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, ctor_ref, &mut ctor) != napi::sys::Status::napi_ok {
        return Err(make_js_error("Error getting constructor."));
    }

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

        let native_object = RequestBlob::new_empty_with_js();
        let raw_obj = Box::into_raw(native_object);

        let _result = sys::napi_wrap(
            env,
            result,
            raw_obj as *mut c_void,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            );

        let recovered = Box::from_raw(raw_obj);
        pool.push(StoredPair((recovered, reffering)));
    }

    Ok(())
}

pub unsafe fn build_up_pool(env: sys::napi_env, pool_size: usize) -> Result<()> {
    let thread_id = crate::thread::get_id(); 
    build_pool_for_id(env, pool_size, thread_id as usize, false)
}


