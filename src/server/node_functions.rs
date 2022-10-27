use napi::bindgen_prelude::*;

use crate::napi::halfbrown::HalfBrown;
use super::{actix_server::start_server, config::ServerConfig, shutdown::stop_server};

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
    config.worker_threads = workers as usize;

    start_server(config, env.raw())
}


#[cold]
#[napi]
/// This is called to start the server the address will need to include the IP and port
/// This allows you to configure more of the parameters of the server current options are all options need to be strings:
/// 
/// url: The url to listen on
/// 
/// worker_threads: The number of worker threads to use
/// 
/// backlog: The number of connections to queue up
/// 
/// pool_per_worker_size: The size of the pool per worker
/// 
/// debug: Whether to enable debug mode
pub fn start_with_config(env: Env, config: HalfBrown<String, String>) -> Result<()> {
    let config = ServerConfig::from_config_blob(config.0)?;

    start_server(config, env.raw())
}

#[cold]
#[napi]
/// Attempts to stop the server, returns if it woreked
/// Experimental at the moment
pub fn stop() -> bool {
    stop_server(true)
}