mod v8_runna;
mod v8;


fn main() {
    let file_name = get_args();

    println!("File name {}", file_name);

    let source = std::fs::read_to_string(&file_name)
    .unwrap_or_else(|err| panic!("failed to open {}: {}", file_name, err));

    println!("Source is {}", source);
    v8_runna::do_some_cray_shit(source)
}

fn get_args() -> String {
    use std::env;
    let args: Vec<String> = env::args().collect();

    args[1].clone()
}