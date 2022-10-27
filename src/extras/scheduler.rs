#[cfg(any(target_os = "linux", target_os = "windows"))]
use lazy_static::lazy_static;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use parking_lot::Mutex;

struct CoreAffinityTracker {
    curr_index: usize,
    min: usize,
    max: usize,
    jump: usize,
}

impl CoreAffinityTracker {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn new() -> Self {
        let cores = affinity::get_core_num();
        let physical = num_cpus::get_physical();

        let mut jump = 1;
        if physical < cores {
            jump = 2;
        }

        Self {
            curr_index: 0,
            min: 0,
            max: cores,
            jump,
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
lazy_static! {
    static ref TRACKER: Mutex<CoreAffinityTracker> = {
        let tracker = CoreAffinityTracker::new();
        Mutex::new(tracker)
    };
}

#[cold]
#[inline(never)]
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn reset_thread_affinity() {
    let mut tracker = TRACKER.lock();

    tracker.min = 0;
    tracker.max = affinity::get_core_num();
    tracker.curr_index = 0;
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn reset_thread_affinity() {}

#[cold]
#[inline(never)]
pub fn pin_js_thread() {
    pin_thread_inner();
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
#[cold]
#[inline(never)]
fn pin_thread_inner() {
    let mut tracker = TRACKER.lock();

    if tracker.min + tracker.jump == tracker.max {
        println!("Ran out of priority cores...");
        drop(tracker);
        try_pin_non_priority();
        return;
    }

    let to_set = tracker.min;
    tracker.min += tracker.jump;

    if tracker.curr_index < tracker.min {
        tracker.curr_index = tracker.min;
    }

    if affinity::set_thread_affinity(&[to_set]).is_err() {
        println!("Error getting afinity");
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn pin_thread_inner() {}

#[cold]
#[inline(never)]
pub fn try_pin_priority() {
    pin_thread_inner()
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
#[cold]
#[inline(never)]
pub fn try_pin_non_priority() {
    let mut tracker = TRACKER.lock();

    let to_set = tracker.curr_index;
    tracker.curr_index += 1;
    if tracker.curr_index >= tracker.max {
        tracker.curr_index = tracker.min;
    }

    if affinity::set_thread_affinity(&[to_set]).is_err() {
        println!("werror getting afinity");
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn try_pin_non_priority() {}