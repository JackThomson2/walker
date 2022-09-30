use std::thread;

use crate::v8::State;

pub fn do_some_cray_shit(function: String) {
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  thread::scope(|s| {
    for _ in 0..64 {
      let f = function.clone();
      s.spawn(move || run_in_thread(f));
    }
  });
}

pub fn do_less_crazy_shit(function: String) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
  
    run_in_thread(function);
  }
  

fn run_in_thread(function: String) {
  let mut isolate = v8::Isolate::new(v8::CreateParams::default());
  let mut scope = v8::HandleScope::new(&mut isolate);

  let source = v8::String::new(&mut scope, &function).unwrap();

  let mut state_machine = State::new(&mut scope, source);

  state_machine.register_listeners();

  state_machine.run_script();
  state_machine.load_found_router();
}
