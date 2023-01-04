use std::sync::atomic::AtomicU32;

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn get_id() -> u32 {
    THREAD_ID.with(|a| *a)
}

thread_local! {
    static THREAD_ID: u32 = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}


