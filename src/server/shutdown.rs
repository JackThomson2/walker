use std::sync::atomic::{AtomicBool, Ordering};

use actix_server::ServerHandle;
use parking_lot::Mutex;

#[derive(Default)]
struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
    running: AtomicBool,
}

impl StopHandle {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
            running: AtomicBool::new(false),
        }
    }

    /// Sets the server handle to stop.
    pub fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub fn stop(&self, graceful: bool) -> bool {
        let mut lock = self.inner.lock();

        let found = match lock.as_ref() {
            Some(handle) => handle,
            None => {
                println!("Trying to stop server but it isn't running!");
                return false
            },
        };

        extreme::run(async move {
            found.stop(graceful).await
        });

        *lock = None;

        true
    }
}

static STOP_HANDLE: StopHandle = StopHandle::new();

pub fn attach_server_handle(handle: ServerHandle) {
    STOP_HANDLE.register(handle);
}

pub fn stop_server(graceful: bool) -> bool {
    if !STOP_HANDLE.running.load(Ordering::SeqCst) {
        return false;
    }

    STOP_HANDLE.stop(graceful)
}

pub fn try_own_start() -> bool {
    STOP_HANDLE.running.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok()
}