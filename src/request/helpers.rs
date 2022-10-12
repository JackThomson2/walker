use actix_http::{header::HeaderMap, Payload};
use bytes::BytesMut;
use futures::StreamExt;
use halfbrown::HashMap;
use napi::{Error, Result, Status};

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

        params.insert(key.to_string(), val.to_string());
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
pub fn load_body_from_payload(mut payload: Payload) -> Result<bytes::Bytes> {
    extreme::run(async move {
        let mut bytes = BytesMut::with_capacity(1024);

        while let Some(item) = payload.next().await {
            let item = item.map_err(|_| make_js_error("Error reading payload"))?;
            bytes.extend_from_slice(&item);
        }

        Ok(bytes.freeze())
    })
}