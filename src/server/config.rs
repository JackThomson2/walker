use std::cmp;

#[napi(object)]
#[derive(Debug)]
/// This allows you to configure the server with more
/// granular control, some options are required.
pub struct ServerConfig {
    pub url: String,
    pub worker_threads: u32,
    pub pool_per_worker_size: u32,
    pub backlog: u32,
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
            worker_threads: guess_optimal_worker_count() as u32,
            pool_per_worker_size: 10_000,
            backlog: 1024,
            debug: false,
            tls: false,
            key_location: None,
            cert_location: None,
        }
    }

    pub fn get_pool_size(&self) -> usize {
        (self.worker_threads * self.pool_per_worker_size) as usize 
    }
}
