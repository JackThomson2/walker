use napi::{sys, Result, Env};

use crate::object_pool::build_pool_for_id;
use super::thread_identifier::get_id;

#[napi]
pub fn get_worker_id() -> u32 {
    get_id()
}

#[napi]
pub fn initialise_pool_for_worker(env: Env, pool_size: u32) -> Result<()> {
   unsafe { build_pool_for_id(env.raw(), pool_size as usize, get_worker_id() as usize) }
}
