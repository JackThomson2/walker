use napi::sys;
use parking_lot::Mutex;

use crate::RequestBlob;

use super::unsafe_impl::get_ctor;

struct Reffer(pub sys::napi_ref);

unsafe impl Send for Reffer {}

static POOL: Mutex<Vec<Reffer>> = Mutex::new(vec![]);

const INITIAL_POOL_SIZE: usize = 50_000;

pub unsafe fn get_from_pool(env: sys::napi_env) -> (sys::napi_ref, sys::napi_value) {
    let mut locked = POOL.lock();
    let obj = locked.pop().unwrap();

    let mut result = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, obj.0, &mut result) != sys::Status::napi_ok {
        panic!("Can't get value");
    }

    (obj.0, result)
}

pub unsafe fn reclaim_ref(env: sys::napi_env, reference: sys::napi_ref) {
    let mut result = std::ptr::null_mut();
    if sys::napi_get_reference_value(env, reference, &mut result) != sys::Status::napi_ok{
        panic!("Can't get value");
    }

    let mut res = std::ptr::null_mut();
    if sys::napi_remove_wrap(env, result, &mut res) != sys::Status::napi_ok{
        panic!("Can't unwrap value");
    }
    let data = *Box::from_raw(res as *mut RequestBlob);
    drop(data);

    let mut locked_pool = POOL.lock();
    locked_pool.push(Reffer(reference));
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
            return;
        }

        let mut reffering = std::ptr::null_mut();
        if sys::napi_create_reference(env, result, 1, &mut reffering) != sys::Status::napi_ok {
            return;
        }

        locked_pool.push(Reffer(reffering))
    }

    println!("Pooled objects build sucessfully");
}
