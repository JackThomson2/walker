use std::ptr;

use bytes::Bytes;
use napi::{
    bindgen_prelude::FromNapiValue,
    check_status,
    sys::{self, napi_env, napi_value},
    Result, TypedArrayType, Error, Status,
};

pub struct JsBytes(pub bytes::Bytes);

impl FromNapiValue for JsBytes {
    #[inline(always)]
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
        let mut typed_array_type = 0;
        let mut length = 0;
        let mut data = ptr::null_mut();
        let mut array_buffer = ptr::null_mut();
        let mut byte_offset = 0;

        check_status!(
            sys::napi_get_typedarray_info(
                env,
                napi_val,
                &mut typed_array_type,
                &mut length,
                &mut data,
                &mut array_buffer,
                &mut byte_offset,
            ),
            "Get TypedArray info failed"
        )?;

        if typed_array_type != TypedArrayType::Uint8 as i32 {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Expected utf8, got {}", typed_array_type),
            ));
        }

        let buffer = Bytes::copy_from_slice(&*ptr::slice_from_raw_parts(data as *mut u8, length));
        Ok(Self(buffer))
    }
}
