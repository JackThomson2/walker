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
        
        Ok(Self {
            url,
            worker_threads: get_number_with_deault("worker_threads", guess_optimal_worker_count())?,
            pool_per_worker_size: get_number_with_deault("pool_per_worker_size", 10_000)?,
            backlog: get_number_with_deault("backlog", 1024)?,
            debug: get_bool_with_default("debug", false)?,
        })
    }

    pub fn get_pool_size(&self) -> usize {
        self.worker_threads * self.pool_per_worker_size
    }
}