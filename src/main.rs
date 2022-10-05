use std::thread;

use flume::{Receiver, Sender};
use messages::request::HttpRequest;
use tokio::net::TcpListener;

mod http;
mod messages;
mod v8;
mod v8_runna;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() {
  let file_name = get_args();

  let source = std::fs::read_to_string(&file_name)
    .unwrap_or_else(|err| panic!("failed to open {}: {}", file_name, err));

  v8_runna::start_platform();

  let (sender, reciever) = flume::bounded(10_000);

  threaded_reciever(source, reciever);
  regular(sender);
}

fn threaded_reciever(source: String, reciever: Receiver<HttpRequest>) {
  for _ in 0..200 {
    let code = source.clone();
    let recv = reciever.clone();

    thread::spawn(move || v8_runna::run_in_thread(code, recv));
  }

  drop(reciever)
}

fn regular(sender: Sender<HttpRequest>) {
  http::start_http_reciever(sender).unwrap();
}

fn get_args() -> String {
  use std::env;
  let args: Vec<String> = env::args().collect();

  args[1].clone()
}
