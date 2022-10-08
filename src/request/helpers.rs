use std::collections::HashMap;

use actix_http::{header::HeaderMap, Payload};
use bytes::BytesMut;
use napi::{Error, Status};
use futures::StreamExt;

#[cold]
#[inline(never)]
pub fn make_generic_error() -> Error {
    Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
    )
}

#[inline(always)]
pub fn split_and_get_query_params(query_string: String) -> HashMap<String, String> {
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

    params
}

#[inline(always)]
pub fn convert_header_map(header_val: &HeaderMap) -> HashMap<String, String> {
    let mut return_map = HashMap::with_capacity(header_val.len());

    for (key, value) in header_val.iter() {
        let string_val = match value.to_str() {
            Ok(res) => res,
            Err(_) => continue,
        };

        return_map.insert(key.to_string(), string_val.to_string());
    }

    return_map
}

#[inline(always)]
pub fn load_body_from_payload(mut payload: Payload) -> bytes::Bytes {
    extreme::run(async move {
        let mut bytes = BytesMut::with_capacity(1024);

        while let Some(item) = payload.next().await {
            let item = item.unwrap();
            bytes.extend_from_slice(&item);
        }

        bytes.freeze()
    })
}
