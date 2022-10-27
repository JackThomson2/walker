use std::ptr;

use napi::{sys::{napi_env, napi_value}, bindgen_prelude::*, Result};
use serde_json::{Value, Map};

use crate::request::helpers::make_js_error;

use super::fast_str::FastStr;

pub struct FasterValue(pub Value);

impl FromNapiValue for FasterValue {
    #[inline(always)]
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
        let obj = Object::from_napi_value(env, napi_val)?;

        let mut names = ptr::null_mut();
        check_status!(
            sys::napi_get_property_names(obj.0.env, obj.0.value, &mut names),
            "Failed to get property names of given object"
        )?;

        let names = Array::from_napi_value(obj.0.env, names)?;
        let mut map = Map::with_capacity(names.len() as usize);

        for i in 0..names.len() {
            let key = names
                .get::<FastStr>(i)?
                .ok_or_else(|| make_js_error("Value not found"))?;

            if let Some(val) = obj.get(&key.0)? {
                map.insert(key.0, val);
            }
        }

        Ok(FasterValue(Value::Object(map)))
    }
}