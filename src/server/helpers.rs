use std::convert::Infallible;

use actix_http::{Response, Payload};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[cold]
#[inline(never)]
pub fn get_failed_message() -> Result<Response<Bytes>, Infallible> {
    Ok(Response::with_body(
        http::StatusCode::NOT_FOUND,
        Bytes::new(),
    ))
}

#[cold]
#[inline(never)]
pub async fn get_post_body(payload: &mut Payload) -> Result<Bytes, &'static str> {
    let mut body = BytesMut::with_capacity(1024);

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|_| "Error reading body")?;

        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err("overflow");
        }

        body.extend_from_slice(&chunk);
    }

    Ok(body.freeze())
}