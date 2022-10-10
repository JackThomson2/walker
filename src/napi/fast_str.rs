use std::{mem, ptr};

use napi::{
    bindgen_prelude::{FromNapiValue, TypeName},
    check_status,
    sys::{self, napi_env, napi_value},
    Error, Result, Status, ValueType,
};
use simdutf8::basic::from_utf8;

pub struct FastStr(pub String);

impl FromNapiValue for FastStr {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
        const FAST_PATH_LEN: usize = 128;
        let mut ret = Vec::with_capacity(FAST_PATH_LEN);
        let mut len = 0;
        let buf_ptr = ret.as_mut_ptr();

        check_status!(
            sys::napi_get_value_string_utf8(env, napi_val, buf_ptr as *mut i8, FAST_PATH_LEN, &mut len),
            "Failed to convert napi `string` into rust type `String`",
        )?;

        if len >= FAST_PATH_LEN - 1 {
            check_status!(
                sys::napi_get_value_string_utf8(env, napi_val, ptr::null_mut(), 0, &mut len),
                "Failed to convert napi `string` into rust type `String`",
            )?;
    
            len += 1;
            ret.reserve(len - FAST_PATH_LEN);

            check_status!(
                sys::napi_get_value_string_utf8(env, napi_val, buf_ptr, len, &mut len),
                "Failed to convert napi `string` into rust type `String`"
            )?;
        }

        let mut ret = mem::ManuallyDrop::new(ret);
        let buf_ptr = ret.as_mut_ptr();
        let bytes = { Vec::from_raw_parts(buf_ptr as *mut u8, len, len) };

        if from_utf8(&bytes).is_err() {
            return Err(Error::new(
                Status::InvalidArg,
                format!("Failed to read utf8 string,",),
            ));
        };

        let new_str = String::from_utf8_unchecked(bytes);
        Ok(Self(new_str))
    }
}

impl TypeName for FastStr {
    fn type_name() -> &'static str {
        "String"
    }

    fn value_type() -> ValueType {
        ValueType::String
    }
}
