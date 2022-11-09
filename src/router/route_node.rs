use crate::napi::tsfn::ThreadsafeFunction;

#[derive(Clone)]
pub struct RouteNode {
    pub threads_id: usize,
    pub function: ThreadsafeFunction
}

impl RouteNode {

    pub fn new_with_fn(function: ThreadsafeFunction) -> Self {
        let thread_id = crate::thread::get_id();

        Self {
            threads_id: thread_id as usize,
            function
        }
    }
}
