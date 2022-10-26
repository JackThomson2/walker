use lazy_static::lazy_static;
use parking_lot::Mutex;

struct CoreAffinityTracker {
    curr_index: usize,
    min: usize,
    max: usize,
    jump: usize,
}

impl CoreAffinityTracker {
    pub fn new() -> Self {
        let cores = affinity::get_core_num();
        let physical = num_cpus::get_physical();

        let mut jump = 1;
        if physical < cores {
            println!("Hyperthreading enabled, pinning to each physical core");
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

lazy_static! {
    static ref TRACKER: Mutex<CoreAffinityTracker> = {
        let tracker = CoreAffinityTracker::new();
        Mutex::new(tracker)
    };
}

#[cold]
#[inline(never)]
pub fn pin_js_thread() {
    pin_thread_inner();
}
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

#[cold]
#[inline(never)]
pub fn try_pin_priority() {
    pin_thread_inner()
}

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