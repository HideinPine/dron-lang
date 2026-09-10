use std::{env, fs};

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = fs::read_to_string(file_path).expect("Please provide a valid file path");
    file
}

fn main() {
    let file = get_file();
    println!("{}", file);
}
