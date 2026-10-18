#![allow(unused)]
use crate::errors::combined::DronError;
use crate::lexer::tokenizer;
use crate::parser::stmt::analyse;
use owo_colors::OwoColorize;
use std::{env, fs, time::Instant};
mod errors;
mod lexer;
mod parser;
mod semantic;

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    //let file_path = args.get(1).map(|s| s.as_str());
    let path: &str = match file_path {
        Some(file) => file.as_str(),
        None => {
            //println!("\nNo file path provided... \nRunning default..\n");
            "../lang-syntax/main.rn"
        }
    };
    //let file_path = &args[1];
    let file: Result<String, std::io::Error> = fs::read_to_string(path);
    match file {
        Ok(string) => string,
        Err(..) => String::from("../lang-syntax/main.rn"),
    }
}

fn to_char(file: String) -> Vec<char> {
    file.chars().collect()
}

fn main() -> Result<(), DronError> {
    let start = Instant::now();
    let file = get_file();
    //println!("{}", file);
    let chars = to_char(file);
    let tokenized = match tokenizer::tokenize(chars) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    };

    match analyse(tokenized) {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1)
        }
    }
    let end = Instant::now();
    println!("\t{} in {:?}", "Finished".bold().green(), end - start);
    Ok(())
}
