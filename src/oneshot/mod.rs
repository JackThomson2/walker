use std::{
  cell::UnsafeCell,
  mem::MaybeUninit,
  ptr,
  sync::{
    atomic::{
      AtomicU8,
      Ordering::{Acquire, SeqCst},
    },
    Arc,
  }, hint,
};

use may::{sync::Blocker, coroutine};

mod states {
  /// The initial channel state. Active while both endpoints are still alive, no message has been
  /// sent, and the receiver is not receiving.
  pub const EMPTY: u8 = 0b011;
  /// A message has been sent to the channel, but the receiver has not yet read it.
  pub const MESSAGE: u8 = 0b100;
  /// No message has yet been sent on the channel, but the receiver is currently receiving.
  pub const RECEIVING: u8 = 0b000;
  /// The channel has been closed. This means that either the sender or receiver has been dropped,
  /// or the message sent to the channel has already been received. Since this is a oneshot
  /// channel, it is disconnected after the one message it is supposed to hold has been
  /// transmitted.
  pub const DISCONNECTED: u8 = 0b010;
}
use states::*;

struct OneShotInner<T> {
  state: AtomicU8,
  message: UnsafeCell<MaybeUninit<T>>,
  waker: UnsafeCell<MaybeUninit<Arc<Blocker>>>,
}

impl<T> OneShotInner<T> {
  pub fn new() -> Self {
    Self {
      state: AtomicU8::new(EMPTY),
      message: UnsafeCell::new(MaybeUninit::uninit()),
      waker: UnsafeCell::new(MaybeUninit::uninit()),
    }
  }

  #[inline(always)]
  unsafe fn store_message(&self, msg: T) {
    let message = &mut *self.message.get();
    message.as_mut_ptr().write(msg)
  }

  #[inline(always)]
  unsafe fn take_message(&self) -> T {
    ptr::read(self.message.get()).assume_init()
  }

  #[inline(always)]
  unsafe fn store_blocker(&self, blocker: Arc<Blocker>) {
    let waker = &mut *self.waker.get();
    waker.as_mut_ptr().write(blocker)
  }

  #[inline(always)]
  unsafe fn wake_inner(&self) {
    let waker = ptr::read(self.waker.get()).assume_init();
    waker.unpark()
  }
}

pub fn channel<T>() -> (Sender<T>, Reciever<T>) {
  let inner = Arc::new(OneShotInner::new());

  (
    Sender {
      inner: inner.clone(),
    },
    Reciever {
      inner,
    },
  )
}

pub struct Reciever<T> {
  inner: Arc<OneShotInner<T>>,
}

impl<T> Reciever<T> {
  #[inline(always)]
  fn check_read(&self) -> Option<T> {
    match self.inner.state.load(Acquire) {
      MESSAGE => Some(unsafe { self.inner.take_message() }),
      _ => None,
    }
  }

  pub fn recv(&self) -> Option<T> {
    // Spin at the start
    for i in 1..5 {
        if let Some(res) = self.check_read() {
            return Some(res);
        }
        
        for _ in 0..i {
            hint::spin_loop();
        }
    }

    for i in 1..3 {
        if let Some(res) = self.check_read() {
            return Some(res);
        }
        for _ in 0..i {
            coroutine::yield_now()
        }
    }

    let cur = Blocker::current();
    unsafe { self.inner.store_blocker(cur.clone()) }

    loop {
      match self.inner.state.load(Acquire) {
        EMPTY => {
          if self
            .inner
            .state
            .compare_exchange(EMPTY, RECEIVING, SeqCst, SeqCst)
            .is_err()
          {
            continue;
          }

          cur.park(None).ok();
          return Some(unsafe { self.inner.take_message() });
        }
        MESSAGE => return Some(unsafe { self.inner.take_message() }),
        RECEIVING => unreachable!("Can't have two recievers"),
        DISCONNECTED => {
          return None;
        }
        _ => unreachable!("Unknown state"),
      }
    }
  }
}

#[derive(Clone)]
pub struct Sender<T> {
  inner: Arc<OneShotInner<T>>,
}

impl<T> Sender<T> {
  pub unsafe fn send(&self, message: T) {
    self.inner.store_message(message);

    loop {
      match self.inner.state.load(Acquire) {
        EMPTY => {
          match self
            .inner
            .state
            .compare_exchange(EMPTY, MESSAGE, SeqCst, SeqCst)
          {
            Ok(_) => return,
            Err(_) => continue,
          }
        }
        MESSAGE => unreachable!("Can't have two people sending messages.."),
        RECEIVING => {
          self.inner.wake_inner();
          return;
        }
        DISCONNECTED => return,
        _ => unreachable!("Unknown state"),
      }
    }
  }
}
