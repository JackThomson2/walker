use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::Ordering};

use napi::{sys, Result};


use super::{helpers::make_js_error};

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
pub fn get_ctor() -> sys::napi_ref {
    unsafe { *(*CONSTRUCTOR.0.get()).as_ptr() }
}

// #[inline(always)]
// pub unsafe fn convert_to_napi(
//     env: sys::napi_env,
//     wrapped_value: *mut c_void,
// ) -> Option<(sys::napi_value, sys::napi_ref)> {
//     let (reffer, value) = get_from_pool(env);
//     let result = sys::napi_wrap(
//         env,
//         value,
//         wrapped_value,
//         None,
//         std::ptr::null_mut(),
//         std::ptr::null_mut(),
//     );
    
//     if result != sys::Status::napi_ok
//     {
//         return None;
//     }
//     Some((value, reffer))
// }
