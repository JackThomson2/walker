use napi::bindgen_prelude::*;

use super::{actix_server::start_server, config::ServerConfig};

#[cold]
#[napi]
/// This is called to start the server the address will need to include the IP and port
/// e.g. localhost:8080
pub fn start(env: Env, address: String) -> Result<()> {
    let config = ServerConfig::default_with_url(address);
    start_server(config, env.raw())
}

#[cold]
#[napi]
/// This is called to start the server the address will need to include the IP and port
/// This allows you to configure the number of workers
pub fn start_with_worker_count(env: Env, address: String, workers: u32) -> Result<()> {
    let mut config = ServerConfig::default_with_url(address);
    config.worker_threads = Some(workers);

    start_server(config, env.raw())
}


#[cold]
#[napi]
/// This is called to start the server the using the `ServerConfig` object
pub fn start_with_config(env: Env, config: ServerConfig) -> Result<()> {
    start_server(config, env.raw())
}

#[cold]
#[napi]
/// Attempts to stop the server, returns if it woreked
/// Experimental at the moment
pub fn stop() -> bool {
    // stop_server(true)
    false
}
