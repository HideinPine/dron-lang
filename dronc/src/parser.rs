#![allow(unused)]
pub mod functione;
pub mod parser{
  use crate::lexer::types::{LexerCartegories, TokenKeyword, TokenSeparator};
  pub enum Def {
    Func(FunDef),
  }
  pub struct FunDef {
    keyword: TokenKeyword,
    fname: String,
    args: Option<Vec<LexerCartegories>>,
    body: Option<Vec<LexerCartegories>>,
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
    use crate::parser::process::peek;

  pub fn keyword_type(keyword: TokenKeyword, token_vec: &Vec<LexerCartegories>, position: usize) {
    match keyword {
      TokenKeyword::Enum => unimplemented!(),
      TokenKeyword::Function => get_values(token_vec, position),
      TokenKeyword::Struct => unimplemented!(),
    }
  }
  pub fn get_values(token_vec: &Vec<LexerCartegories>, position: usize){
    let pos = position;
    let name = peek(pos,&token_vec);
    match name {
      Some(LexerCartegories::Identifier(token_identifier)) => println!("Found: {:?}", token_identifier),
      Some(LexerCartegories::Separator(_)) => println!("Found {:?}",name.unwrap()),
      Some(LexerCartegories::Keyword(_)) => println!("Found {:?}",name.unwrap()),
      Some(LexerCartegories::EndOfFile) => println!("Found {:?}",name.unwrap()),
      Some(LexerCartegories::Literal(_)) => println!("Found {:?}",name.unwrap()),
      Some(LexerCartegories::Operator(_)) => println!("Found {:?}",name.unwrap()),
      None => println!("None found"), 
    }
  }
  
}