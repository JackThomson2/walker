use napi::bindgen_prelude::*;

#[napi]
pub fn start(address: String) -> Result<()> {
    crate::server::internal::start_server(address);

    Ok(())
}