use napi::bindgen_prelude::*;

#[cold]
#[napi]
/// This is called to start the server the address will need to include the IP and port
/// e.g. localhost:8080
pub fn start(address: String, workers: u32) -> Result<()> {
    crate::server::actix_server::start_server(address, workers as usize);

    Ok(())
}