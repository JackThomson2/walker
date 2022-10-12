use futures::Future;
use tokio::runtime::{Runtime, Builder};
use lazy_static::lazy_static;

lazy_static!{
    pub(crate) static ref RUNNER: Runtime = {
        Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("walker-worker")
            .enable_all()
            .build()
            .unwrap()
    };
}

pub fn spawn<F>(fut: F)
where
  F: 'static + Send + Future<Output = ()>,
{
  RUNNER.spawn(fut);
}