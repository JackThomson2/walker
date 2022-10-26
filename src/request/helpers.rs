use actix_http::{header::HeaderMap};
use bytes::{BytesMut, Bytes, BufMut};
use halfbrown::HashMap;
use napi::{Error, Result, Status};
use serde_json::Value;

use crate::napi::halfbrown::HalfBrown;

#[cold]
#[inline(never)]
pub fn make_generic_error() -> Error {
    Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
    )
}

#[cold]
#[inline(never)]
pub fn make_js_error(reason: &'static str) -> Error {
    Error::new(Status::GenericFailure, reason.to_string())
}

#[cold]
#[inline(never)]
pub fn make_js_error_string(reason: String) -> Error {
    Error::new(Status::GenericFailure, reason)
}

#[inline(always)]
pub fn split_and_get_query_params(query_string: String) -> HalfBrown<String, String> {
    let mut params = HashMap::with_capacity(16);
    for pair in query_string.split('&') {
        let mut split_two = pair.split('=');

        let key = match split_two.next() {
            Some(res) => res,
            None => continue,
        };

        let val = match split_two.next() {
            Some(res) => res,
            None => continue,
        };

        params.insert(key.to_owned(), val.to_owned());
    }

    HalfBrown(params)
}

#[inline(always)]
pub fn convert_header_map(header_val: &HeaderMap) -> HalfBrown<String, String> {
    let mut return_map = HashMap::with_capacity(header_val.len());

    for (key, value) in header_val.iter() {
        let string_val = match value.to_str() {
            Ok(res) => res,
            Err(_) => continue,
        };

        return_map.insert(key.to_string(), string_val.to_string());
    }

    HalfBrown(return_map)
}

#[inline(always)]
pub fn value_to_bytes(value: Value) -> Result<Bytes> {
    let bytes = BytesMut::with_capacity(128);
    let mut writer = bytes.writer();
    serde_json::to_writer(&mut writer, &value)
        .map_err(|_| make_js_error("Error serialising data."))?;

    let bytes = writer.into_inner();

    Ok(bytes.freeze())
}
