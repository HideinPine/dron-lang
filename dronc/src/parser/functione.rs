pub mod name_error {
  use crate::lexer::types::LexerCartegories;
  use owo_colors::OwoColorize;
  pub fn function_name_error(value: Option<&LexerCartegories>, pos: usize) {
    println!("{}{:?}{}{}{}", "Error Found: ".bold().red(), value.unwrap().bold().red()," instead of function identifier".bold().red(), " at position: ".red().bold(), pos.bold());
  }
}