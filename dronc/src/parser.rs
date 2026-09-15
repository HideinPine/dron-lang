#![allow(unused)]
pub mod functione;
pub mod parser{
  use crate::lexer::types::{LexerCartegories, TokenKeyword, TokenSeparator};
  use crate::parser::match_access::function_values;
  pub enum Def {
    Func(FunDef),
  }
  pub struct FunDef {
    keyword: TokenKeyword,
    fname: String,
    args: Option<Vec<LexerCartegories>>,
    body: Option<Vec<LexerCartegories>>,
  }
  impl FunDef{
    fn new() -> Self {
      FunDef {
        keyword: TokenKeyword::Function,
        fname: String::from("main"),
        args: None,
        body: None,
      }
    }
  }
  pub fn try_val(token_vec: &Vec<LexerCartegories>) {
    for (i, token) in token_vec.iter().enumerate() {
      let eof = LexerCartegories::EndOfFile;
      if *token == eof {
        /*token_vec.push(FunDef::new());*/
        println!("EndOfFile reached");
        break;
      } else if *token == LexerCartegories::Keyword(TokenKeyword::Function){
        function_values(&token_vec, i);
      } else {
        continue;
      }
    }
  }
}

pub mod process{
  use std::clone;
  use crate::{lexer::types::{LexerCartegories}, parser::parser::{Def}};
  pub fn consume(position: usize, token_vec: &Vec<LexerCartegories>) -> LexerCartegories {
    <LexerCartegories as Clone>::clone(&token_vec[position])
  }
  pub fn peek<'a>(postion: usize, token_vec: &'a Vec<LexerCartegories>) -> Option<&'a LexerCartegories> {
    token_vec.get(postion)
  }
  pub fn iseof(token_vec: &Vec<LexerCartegories>, position: usize) -> bool {
    let eof: LexerCartegories = LexerCartegories::EndOfFile;
    token_vec[position] == eof
  }
}
pub mod match_access{
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    use crate::parser::process::{consume, peek};
    use crate::functione::name;

  fn keyword_type(keyword: TokenKeyword, token_vec: &Vec<LexerCartegories>, position: usize) {
    let pos = position + 1;
    match keyword {
      TokenKeyword::Enum => get_values(token_vec,pos),
      TokenKeyword::Function => get_values(token_vec, pos),
      TokenKeyword::Struct => get_values(token_vec, pos),
    } 
  }
  pub fn function_values(token_vec: &Vec<LexerCartegories>, position: usize) {
    keyword_type(TokenKeyword::Function, token_vec, position);
  }
  fn get_values(token_vec: &Vec<LexerCartegories>, position: usize){
    let pos = position;
    let name = peek(pos,&token_vec);
    match name {
      Some(LexerCartegories::Identifier(token_identifier)) => println!("Found: {:?}", token_identifier),
      Some(LexerCartegories::Separator(_)) => name::function_name_error(name),
      Some(LexerCartegories::Keyword(_)) => name::function_name_error(name),
      Some(LexerCartegories::EndOfFile) => name::function_name_error(name),
      Some(LexerCartegories::Literal(_)) => name::function_name_error(name),
      Some(LexerCartegories::Operator(_)) => name::function_name_error(name),
      None => todo!(), 
    }
  }
}