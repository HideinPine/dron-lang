#![allow(unused)]
use crate::lexer::{checker, tokenizer};
use crate::parser::errors;
use std::{env, fs};
mod lexer;
mod parser;

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    //let file_path = args.get(1).map(|s| s.as_str());
    let path: &str = match file_path {
        Some(file) => file.as_str(),
        None => {
            println!("\nNo file path provided... \nRunning default..\n");
            "../tests/main.rv"
        }
    };
    //let file_path = &args[1];
    let file: Result<String, std::io::Error> = fs::read_to_string(path);
    match file {
        Ok(string) => string,
        Err(..) => String::from("../tests/main.hv"),
    }
}

fn to_char(file: String) -> Vec<char> {
    file.chars().collect()
}

fn main() {
    let file = get_file();
    /* println!("{}", file); */
    let chars = to_char(file);
    tokenizer::tokenize(chars);
}
