use std::{ffi::CString, ptr};

use napi::{
    bindgen_prelude::{Array, FromNapiValue},
    check_status, sys, Env, JsObject, Result,
};

use crate::napi::fast_str::FastStr;

#[napi]
pub struct ObjectTemplate {
    #[allow(dead_code)]
    pub(crate) keys: Vec<CString>,
}

#[napi]
impl ObjectTemplate {
    #[inline]
    unsafe fn make_self(env: sys::napi_env, obj: JsObject) -> Result<Self> {
        let mut names = ptr::null_mut();
        check_status!(
            sys::napi_get_property_names(env, obj.0.value, &mut names),
            "Failed to get property names of given object"
        )?;

        let names = Array::from_napi_value(obj.0.env, names)?;

        let mut keys = Vec::with_capacity(names.len() as usize);

        for i in 0..names.len() {
            let key = names.get::<FastStr>(i)?.unwrap();
            keys.push(CString::new(key.0)?);
        }

        Ok(Self { keys })
    }

    #[napi]
    pub unsafe fn create_template(env: Env, obj: JsObject) -> Result<Self> {
        ObjectTemplate::make_self(env.raw(), obj)
    }
}
