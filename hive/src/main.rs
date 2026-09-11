use std::{env, fs};

use crate::lexer::tokenizer;
mod lexer;

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = fs::read_to_string(file_path).expect("No file path provided");
    file
}

fn to_char(file: String) -> Vec<char> {
    file.chars().collect()
}

fn main() {
    let file = get_file();
    let chars = to_char(file);
    tokenizer::tokenize_values(chars);
}
