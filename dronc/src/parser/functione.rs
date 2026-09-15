pub mod name {
  use crate::lexer::types::LexerCartegories;
  use owo_colors::OwoColorize;
  pub fn function_name_error(value: Option<&LexerCartegories>) {
    println!("Found {:?} instead of function identifier",value.unwrap().red());
  }
}