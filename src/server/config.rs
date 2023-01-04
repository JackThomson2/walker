use std::cmp;

#[napi(object)]
#[derive(Debug, Default)]
/// This allows you to configure the server with more
/// granular control, some options are required.
pub struct ServerConfig {
    pub url: String,
    pub worker_threads: Option<u32>,
    pub pool_per_worker_size: Option<u32>,
    pub backlog: Option<u32>,
    pub debug: Option<bool>,
    pub tls: Option<bool>,
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
            ..Default::default()
        }
    }

    pub fn get_worker_thread(&self) -> u32 {
        self.worker_threads.unwrap_or(guess_optimal_worker_count() as u32)
    } 
 
    pub fn get_pool_per_worker(&self) -> u32 {
        self.pool_per_worker_size.unwrap_or(10_000)
    }

    pub fn get_backlog_size(&self) -> u32 {
        self.backlog.unwrap_or(1024)
    }

    pub fn get_debug(&self) -> bool {
        self.debug.unwrap_or(false)
    }

    pub fn get_tls(&self) -> bool {
        self.tls.unwrap_or(false)
    }

    pub fn get_pool_size(&self) -> usize {
        (self.get_worker_thread() * self.get_pool_per_worker()) as usize 
    }
}
