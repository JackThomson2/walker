use std::{ffi::CString, ptr};

use napi::{
    bindgen_prelude::ToNapiValue,
    check_status,
    sys::{self, napi_env, napi_value},
    Result,
};
use serde_json::Value;
use tokio_postgres::{
    types::{FromSql, Type},
    Column, Row,
};

use crate::request::helpers::make_js_error_string;

pub struct PostgresData(pub Row);

impl ToNapiValue for PostgresData {
    unsafe fn to_napi_value(raw_env: napi_env, val: Self) -> Result<napi_value> {
        let mut raw_object = ptr::null_mut();
        check_status!(sys::napi_create_object(raw_env, &mut raw_object))?;

        let row = val.0;

        for (i, column) in row.columns().iter().enumerate() {
            let c_field = CString::new(column.name())?;
            let json_value = pg_cell_to_js_value(raw_env, &row, column, i)?;

            check_status!(
                sys::napi_set_named_property(raw_env, raw_object, c_field.as_ptr(), json_value),
                "Failed to set property with field `{}`",
                c_field.to_string_lossy(),
            )?;
        }

        Ok(raw_object)
    }
}

#[inline]
pub unsafe fn pg_cell_to_js_value(
    env: sys::napi_env,
    row: &Row,
    column: &Column,
    column_i: usize,
) -> Result<napi_value> {
    match *column.type_() {
        Type::BOOL => get_basic(env, row, column, column_i, |a: bool| {
            bool::to_napi_value(env, a)
        }),
        Type::INT2 => get_basic(env, row, column, column_i, |a: i16| {
            i16::to_napi_value(env, a)
        }),
        Type::INT4 => get_basic(env, row, column, column_i, |a: i32| {
            i32::to_napi_value(env, a)
        }),
        Type::INT8 => get_basic(env, row, column, column_i, |a: i64| {
            i64::to_napi_value(env, a)
        }),

        Type::FLOAT4 => get_basic(env, row, column, column_i, |a: f32| {
            f32::to_napi_value(env, a)
        }),
        Type::FLOAT8 => get_basic(env, row, column, column_i, |a: f64| {
            f64::to_napi_value(env, a)
        }),

        Type::TEXT | Type::VARCHAR => get_basic(env, row, column, column_i, |a: String| {
            String::to_napi_value(env, a)
        }),
        Type::JSON => get_basic(env, row, column, column_i, |a: Value| {
            Value::to_napi_value(env, a)
        }),

        ref e => {
            println!("Unknown type {}", e.name());
            get_undefined(env)
        },
    }
}

#[inline]
unsafe fn get_basic<'a, T: FromSql<'a>>(
    env: sys::napi_env,
    row: &'a Row,
    column: &Column,
    column_i: usize,
    val_to_json_val: impl Fn(T) -> Result<napi_value>,
) -> Result<napi_value> {
    let raw_val = row
        .try_get::<_, Option<T>>(column_i)
        .map_err(|_| make_js_error_string(format!("Error with column {}", column.name())))?;
    raw_val.map_or_else(|| get_undefined(env), val_to_json_val)
}

#[inline]
unsafe fn get_undefined(env: sys::napi_env) -> Result<sys::napi_value> {
    let mut ret = ptr::null_mut();

    check_status!(
        sys::napi_get_null(env, &mut ret),
        "Failed to create napi null value"
    )?;

    Ok(ret)
}
