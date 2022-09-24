use std::{
  cell::UnsafeCell,
  mem::MaybeUninit,
  sync::atomic::{AtomicBool, Ordering},
};

use may::sync::mpsc::{self, Receiver, Sender};

use super::request::RequestData;

struct MessagePair {
  reciever: Receiver<RequestData>,
  sender: Sender<RequestData>,
}

struct MessagePairHolder(UnsafeCell<MaybeUninit<MessagePair>>);

static INITIALISED: AtomicBool = AtomicBool::new(false);

static STOREDQUEUES: MessagePairHolder =
  MessagePairHolder(unsafe { MaybeUninit::uninit().assume_init() });
unsafe impl Sync for MessagePairHolder {}

/// This MUST be called before recieving any HTTP requests
pub fn init_pair() {
  assert!(!INITIALISED.load(Ordering::SeqCst));

  let (sender, reciever) = mpsc::channel();
  let pair = MessagePair { sender, reciever };

  let global_store = unsafe { &mut *STOREDQUEUES.0.get() };
  *global_store = MaybeUninit::new(pair);

  INITIALISED.store(true, Ordering::SeqCst);
}

#[inline(always)]
fn get_pairs() -> &'static MessagePair {
  unsafe { &*(*STOREDQUEUES.0.get()).as_ptr() }
}

#[inline(always)]
pub fn get_sender() -> Sender<RequestData> {
    let pair = get_pairs();
    pair.sender.clone()
}

#[inline(always)]
pub fn get_reader() -> &'static Receiver<RequestData> {
    let pair = get_pairs();

    &pair.reciever
}