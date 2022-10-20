use std::ffi::c_void;

use napi::sys;
use parking_lot::Mutex;

use crate::RequestBlob;

use super::unsafe_impl::get_ctor;

pub struct StoredPair(pub (Box<RequestBlob>, sys::napi_ref));

unsafe impl Send for StoredPair {}
unsafe impl Sync for StoredPair {}

static POOL: Mutex<Vec<StoredPair>> = Mutex::new(vec![]);

const INITIAL_POOL_SIZE: usize = 100_000;

pub unsafe fn get_from_pool() -> StoredPair {
    let mut locked = POOL.lock();
    let obj = locked.pop().unwrap();

    obj
}

pub unsafe fn get_stored_chunk(count: usize) -> Vec<StoredPair> {
    let mut locked = POOL.lock();
    let split_point = locked.len() - count;

    locked.split_off(split_point)
}

pub unsafe fn build_up_pool(env: sys::napi_env) {
    let ctor_ref = get_ctor();
    let mut ctor = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, ctor_ref, &mut ctor) != napi::sys::Status::napi_ok {
        return;
    }

    let mut locked_pool = POOL.lock();
    locked_pool.reserve(INITIAL_POOL_SIZE);

    println!("Pooling objects");

    for _ in 0..INITIAL_POOL_SIZE {
        let mut result = std::ptr::null_mut();
        if sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result)
            != sys::Status::napi_ok
        {
            println!("Error creatihng a new instace!");
            return;
        }

        let mut reffering = std::ptr::null_mut();
        if sys::napi_create_reference(env, result, 1, &mut reffering) != sys::Status::napi_ok {
            println!("Error creating the reference!");
            return;
        }

        let mut found_obj = std::ptr::null_mut();
        if sys::napi_get_reference_value(env, reffering, &mut found_obj) != sys::Status::napi_ok {
            println!("Error doing the deref!");
            return;
        }

        let native_object = RequestBlob::new_empty_with_js(found_obj);
        let raw_obj = Box::into_raw(native_object);

        let result = sys::napi_wrap(
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

    println!("Pooled objects build sucessfully");
}
