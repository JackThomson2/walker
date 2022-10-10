use std::{hash::Hash, ptr};

use halfbrown::HashMap;
use napi::{
    bindgen_prelude::{Array, FromNapiValue, Object, ToNapiValue, TypeName},
    check_status,
    sys::{self, napi_env, napi_value},
    Env, Result, ValueType,
};

use super::fast_str::FastStr;

pub struct HalfBrown<K, V>(pub HashMap<K, V>);

impl<K, V> FromNapiValue for HalfBrown<K, V>
where
    K: From<String> + Eq + Hash,
    V: FromNapiValue,
{
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
        let obj = Object::from_napi_value(env, napi_val)?;

        let mut names = ptr::null_mut();
        check_status!(
            sys::napi_get_property_names(obj.0.env, obj.0.value, &mut names),
            "Failed to get property names of given object"
        )?;

        let names = Array::from_napi_value(obj.0.env, names)?;
        let mut map = HashMap::with_capacity(names.len() as usize);

        for i in 0..names.len() {
            let key = names.get::<FastStr>(i)?.unwrap();

            if let Some(val) = obj.get(&key.0)? {
                map.insert(K::from(key.0), val);
            }
        }

        Ok(HalfBrown(map))
    }
}

impl<K, V> ToNapiValue for HalfBrown<K, V>
where
    K: AsRef<str>,
    V: ToNapiValue,
{
    unsafe fn to_napi_value(raw_env: napi_env, val: Self) -> Result<napi_value> {
        let env = Env::from(raw_env);
        let mut obj = env.create_object()?;
        for (k, v) in val.0.into_iter() {
            obj.set(k.as_ref(), v)?;
        }

        Object::to_napi_value(raw_env, obj)
    }
}

impl<K, V> TypeName for HalfBrown<K, V> {
    fn type_name() -> &'static str {
        "HalfBrown"
    }

    fn value_type() -> ValueType {
        ValueType::Object
    }
}
