use std::cmp;

use napi::Result;
use halfbrown::HashMap;

use crate::request::helpers::{make_js_error, make_js_error_string};

#[derive(Debug)]
pub struct ServerConfig {
    pub url: String,
    pub worker_threads: usize,
    pub pool_per_worker_size: usize,
    pub backlog: usize,
    pub debug: bool,
    pub tls: bool,
    pub key_location: Option<String>,
    pub cert_location: Option<String>,
}

#[cold]
pub fn guess_optimal_worker_count() -> usize {
    let count = num_cpus::get_physical();
    cmp::max(4, count.saturating_sub(2))
}


impl ServerConfig {
    #[cold]
    pub fn default_with_url(url: String) -> Self {
        Self {
            url,
            worker_threads: guess_optimal_worker_count(),
            pool_per_worker_size: 10_000,
            backlog: 1024,
            debug: false,
            tls: false,
            key_location: None,
            cert_location: None,
        }
    }

    #[cold]
    pub fn from_config_blob(config: HashMap<String, String>) -> Result<Self> {
        let url = match config.get("url") {
            Some(res) => res.clone(),
            None => return Err(make_js_error("No URL provided")),
        };

        let get_number_with_deault = |key: &'static str, fallback: usize| {
            match config.get(key) {
                Some(res) => match res.parse::<usize>() {
                    Ok(res) => Ok(res),
                    Err(_) => Err(make_js_error_string(format!("Invalid number provided for {}", key))),
                },
                None => Ok(fallback),
            }
        };

        let get_bool_with_default = |key: &'static str, fallback: bool| {
            match config.get(key) {
                Some(res) => match res.parse::<bool>() {
                    Ok(res) => Ok(res),
                    Err(_) => Err(make_js_error_string(format!("Invalid number provided for {}", key))),
                },
                None => Ok(fallback),
            }
        };

        let mut tls = false;
        let mut key_location = None;
        let mut cert_location = None;
        
        if get_bool_with_default("tls", false)? {
            tls = true;
            key_location = config.get("key_location").cloned();
            cert_location = config.get("cert_location").cloned();

            if key_location.is_none() || cert_location.is_none() {
                return Err(make_js_error("We need both the key and cert location for TLS"));
            }
        }

        Ok(Self {
            url,
            worker_threads: get_number_with_deault("worker_threads", guess_optimal_worker_count())?,
            pool_per_worker_size: get_number_with_deault("pool_per_worker_size", 10_000)?,
            backlog: get_number_with_deault("backlog", 1024)?,
            debug: get_bool_with_default("debug", false)?,
            tls,
            key_location,
            cert_location,
        })
    }

    pub fn get_pool_size(&self) -> usize {
        self.worker_threads * self.pool_per_worker_size
    }
}
