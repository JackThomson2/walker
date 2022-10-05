use std::{thread, marker::PhantomData};

use flume::Receiver;

use crate::{v8::State, messages::request::HttpRequest};

pub fn start_platform() {
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
}

// pub fn do_some_cray_shit(function: String) {
//   let platform = v8::new_default_platform(0, false).make_shared();
//   v8::V8::initialize_platform(platform);
//   v8::V8::initialize();

//   thread::scope(|s| {
//     for _ in 0..16 {
//       let f = function.clone();
//       s.spawn(move || run_in_thread(f));
//     }
//   });
// }

// pub fn do_less_crazy_shit(function: String) {
//   let platform = v8::new_default_platform(0, false).make_shared();
//   v8::V8::initialize_platform(platform);
//   v8::V8::initialize();

//   run_in_thread(function);
// }

pub fn run_in_thread(function: String, recv: Receiver<HttpRequest>) {
  let mut isolate = v8::Isolate::new(v8::CreateParams::default());
  let mut scope = v8::HandleScope::new(&mut isolate);

  let source = v8::String::new(&mut scope, &function).unwrap();

  let mut state_machine = State::new(&mut scope, source);

  state_machine.register_listeners();

  state_machine.run_script();

  while let Ok(req) = recv.recv() {
    let result = state_machine.find_and_run_route(&req.route);

    let to_send = match result {
      Some(res) => bytes::Bytes::copy_from_slice(res.as_bytes()),
      None => bytes::Bytes::new()
    };

    req.response.send(to_send).ok();
  } 

}
