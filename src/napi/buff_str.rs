use std::ptr;

use bytes::{BytesMut, Bytes};
use napi::{
    bindgen_prelude::{FromNapiValue, TypeName},
    check_status,
    sys::{self, napi_env, napi_value},
    Result, ValueType,
};

/// This is String from JS which is stored in a bytes buffer
pub struct BuffStr(pub Bytes);

impl FromNapiValue for BuffStr {
    #[inline(always)]
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
        const FAST_PATH_LEN: usize = 128;

        // We'll try a fast path here
        let mut buffer = BytesMut::with_capacity(FAST_PATH_LEN);
        let mut len = 0;

        let mut buf_ptr = buffer.as_mut_ptr();
        check_status!(
            sys::napi_get_value_string_utf8(env, napi_val, buf_ptr as *mut _, FAST_PATH_LEN, &mut len),
            "Failed to convert napi `string` into rust type `String`",
        )?;

        // We fallback to slow path if we missed here...
        if len >= FAST_PATH_LEN - 1 {
            check_status!(
                sys::napi_get_value_string_utf8(env, napi_val, ptr::null_mut(), 0, &mut len),
                "Failed to convert napi `string` into rust type `String`",
            )?;
    
            len += 1;
            buffer.reserve(len);
            buf_ptr = buffer.as_mut_ptr();

            check_status!(
                sys::napi_get_value_string_utf8(env, napi_val, buf_ptr as *mut _, len, &mut len),
                "Failed to convert napi `string` into rust type `String`",
            )?;
        }

        buffer.set_len(len);

        Ok(Self(buffer.freeze()))
    }
}

impl TypeName for BuffStr {
    fn type_name() -> &'static str {
        "String"
    }

    fn value_type() -> ValueType {
        ValueType::String
    }
}
