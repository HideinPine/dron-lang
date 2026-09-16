pub mod name {
  use crate::lexer::types::LexerCartegories;
  use owo_colors::OwoColorize;
  pub fn function_name_error(value: Option<&LexerCartegories>) {
    println!("{}{:?}{}", "Error Found, ".bold().red().on_bright_cyan(), value.unwrap().bold().red()," instead of function identifier".bold().red());
  }
}