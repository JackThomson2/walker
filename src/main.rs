use std::{net::TcpListener, thread};

mod http;
mod v8_runna;
mod v8;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() {
    let file_name = get_args();

    println!("File name {}", file_name);

    let source = std::fs::read_to_string(&file_name)
    .unwrap_or_else(|err| panic!("failed to open {}: {}", file_name, err));

    println!("Source is {}", source);
    v8_runna::start_platform();

    regular(source);
}

fn threaded_reciever(source: String) {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    listener.set_nonblocking(true).unwrap();

    thread::scope(|s| {
        for _ in 0..10 {
            let listening = listener.try_clone().unwrap();
            let code = source.clone();

            s.spawn(move || {
                http::start_http_reciever(listening, code).unwrap();
            });
        }
    });
}

fn regular(source: String) {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    listener.set_nonblocking(true).unwrap();

    http::start_http_reciever(listener, source).unwrap();
}

fn get_args() -> String {
    use std::env;
    let args: Vec<String> = env::args().collect();

    args[1].clone()
}