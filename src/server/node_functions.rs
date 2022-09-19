use napi::bindgen_prelude::*;

#[cold]
#[napi]
/// This is called to start the server the address will need to include the IP and port
/// e.g. localhost:8080
pub fn start(address: String) -> Result<()> {
    crate::server::internal::start_server(address);

    Ok(())
}