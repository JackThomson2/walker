use napi::{Result, Env};

use crate::object_pool::build_pool_for_id;
use super::thread_identifier::get_id;

#[napi]
pub fn get_worker_id() -> u32 {
    get_id()
}

#[napi]
/// This is used so you can register a worker thread under walker, this needs to be called
/// before the server starts to register the pool of objects used for requests.
pub fn register_threads_pool(env: Env, pool_size: u32) -> Result<()> {
   unsafe { build_pool_for_id(env.raw(), pool_size as usize, get_worker_id() as usize, true) }
}
