// Fork of threadsafe_function from napi-rs that allows calling JS function manually rather than
// only returning args. This enables us to use the return value of the function.

#![allow(clippy::single_component_path_imports)]

use std::convert::Into;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use napi::{check_status, sys, Result, Status};

#[repr(u8)]
pub enum ThreadsafeFunctionCallMode {
    NonBlocking,
    Blocking,
}

impl From<ThreadsafeFunctionCallMode> for sys::napi_threadsafe_function_call_mode {
    fn from(value: ThreadsafeFunctionCallMode) -> Self {
        match value {
            ThreadsafeFunctionCallMode::Blocking => sys::ThreadsafeFunctionCallMode::blocking,
            ThreadsafeFunctionCallMode::NonBlocking => sys::ThreadsafeFunctionCallMode::nonblocking,
        }
    }
}

/// Communicate with the addon's main thread by invoking a JavaScript function from other threads.
///
/// ## Example
/// An example of using `ThreadsafeFunction`:
///
/// ```rust
/// #[macro_use]
/// extern crate napi_derive;
///
/// use std::thread;
///
/// use napi::{
///     threadsafe_function::{
///         ThreadSafeCallContext, ThreadsafeFunctionCallMode, ThreadsafeFunctionReleaseMode,
///     },
///     CallContext, Error, JsFunction, JsNumber, JsUndefined, Result, Status,
/// };
///
/// #[js_function(1)]
/// pub fn test_threadsafe_function(ctx: CallContext) -> Result<JsUndefined> {
///   let func = ctx.get::<JsFunction>(0)?;
///
///   let tsfn =
///       ctx
///           .env
///           .create_threadsafe_function(&func, 0, |ctx: ThreadSafeCallContext<Vec<u32>>| {
///             ctx.value
///                 .iter()
///                 .map(|v| ctx.env.create_uint32(*v))
///                 .collect::<Result<Vec<JsNumber>>>()
///           })?;
///
///   let tsfn_cloned = tsfn.clone();
///
///   thread::spawn(move || {
///       let output: Vec<u32> = vec![0, 1, 2, 3];
///       // It's okay to call a threadsafe function multiple times.
///       tsfn.call(Ok(output.clone()), ThreadsafeFunctionCallMode::Blocking);
///   });
///
///   thread::spawn(move || {
///       let output: Vec<u32> = vec![3, 2, 1, 0];
///       // It's okay to call a threadsafe function multiple times.
///       tsfn_cloned.call(Ok(output.clone()), ThreadsafeFunctionCallMode::NonBlocking);
///   });
///
///   ctx.env.get_undefined()
/// }
/// ```
pub struct ThreadsafeFunction {
    raw_tsfn: sys::napi_threadsafe_function,
    ref_count: Arc<AtomicUsize>,
}

impl Clone for ThreadsafeFunction {
    fn clone(&self) -> Self {
        Self {
            raw_tsfn: self.raw_tsfn,
            ref_count: Arc::clone(&self.ref_count),
        }
    }
}

unsafe impl Send for ThreadsafeFunction {}
unsafe impl Sync for ThreadsafeFunction {}

impl ThreadsafeFunction {
    /// See [napi_create_threadsafe_function](https://nodejs.org/api/n-api.html#n_api_napi_create_threadsafe_function)
    /// for more information.
    pub(crate) fn create(
        env: sys::napi_env,
        func: sys::napi_value,
        max_queue_size: usize,
    ) -> Result<Self> {
        let mut async_resource_name = ptr::null_mut();
        let s = "napi_rs_threadsafe_function";
        let len = s.len();
        let s = CString::new(s)?;
        check_status!(unsafe {
            sys::napi_create_string_utf8(env, s.as_ptr(), len, &mut async_resource_name)
        })?;

        let initial_thread_count = 1usize;
        let mut raw_tsfn = ptr::null_mut();
        let ptr = ptr::null_mut();
        check_status!(unsafe {
            sys::napi_create_threadsafe_function(
                env,
                func,
                ptr::null_mut(),
                async_resource_name,
                max_queue_size,
                initial_thread_count,
                ptr,
                Some(thread_finalize_cb),
                ptr,
                Some(call_js_cb),
                &mut raw_tsfn,
            )
        })?;

        Ok(ThreadsafeFunction {
            raw_tsfn,
            ref_count: Arc::new(AtomicUsize::new(initial_thread_count)),
        })
    }
}

impl ThreadsafeFunction {
    /// See [napi_call_threadsafe_function](https://nodejs.org/api/n-api.html#n_api_napi_call_threadsafe_function)
    /// for more information.
    #[inline(always)]
    pub fn call(&self, value: sys::napi_ref, mode: ThreadsafeFunctionCallMode) -> Status {
        unsafe { sys::napi_call_threadsafe_function(self.raw_tsfn, value as *mut _, mode.into()) }
            .into()
    }
}

impl Drop for ThreadsafeFunction {
    fn drop(&mut self) {
        if self.ref_count.load(Ordering::Acquire) > 0usize {
            let release_status = unsafe {
                sys::napi_release_threadsafe_function(
                    self.raw_tsfn,
                    sys::ThreadsafeFunctionReleaseMode::release,
                )
            };
            assert!(
                release_status == sys::Status::napi_ok,
                "Threadsafe Function release failed"
            );
        }
    }
}

unsafe extern "C" fn thread_finalize_cb(
    _raw_env: sys::napi_env,
    _finalize_data: *mut c_void,
    _finalize_hint: *mut c_void,
) {
    // cleanup
    //v drop(Box::<R>::from_raw(finalize_data.cast()));
}

#[inline(always)]
unsafe extern "C" fn call_js_cb(
    raw_env: sys::napi_env,
    js_callback: sys::napi_value,
    _context: *mut c_void,
    data: *mut c_void,
) {
    // env and/or callback can be null when shutting down
    if raw_env.is_null() || js_callback.is_null() {
        return;
    }

    let mut recv = ptr::null_mut();
    sys::napi_get_undefined(raw_env, &mut recv);

    let mut found_obj = std::ptr::null_mut();
    if sys::napi_get_reference_value(raw_env, data as sys::napi_ref, &mut found_obj) != sys::Status::napi_ok {
        println!("Error doing the deref!");
        return;
    }

    let args = [found_obj];

    sys::napi_call_function(
        raw_env,
        recv,
        js_callback,
        1,
        args.as_ptr(),
        std::ptr::null_mut(),
    );
}
