#![allow(unused)]
pub mod types{
  pub struct LexerCartegories {
    keyword: TokenKeyword, /* 3rd: PAUSE -- ALMOST DONE */
    separator: TokenSeparator, /* 1st: PAUSE -- ALMOST DONE*/
    literal: TokenLiteral, /* LATER ON WHEN NEEDED */
    operator: TokenOperator, /* 2nd: PAUSE -- ALMOST DONE */
    identifier: TokenIdentifier,
  }
  
  #[derive(Debug)]
  pub enum TokenKeyword{
    Function,
    Enum,
    Struct,
  }
  #[derive(Debug)]
  pub enum TokenSeparator {
    OpenParent,
    CloseParent,
    SemiColon,
    LeftCurl,
    RightCurl,
  }
  enum TokenLiteral {
    True,
    False,
  }
  #[derive(Debug)]
  pub enum TokenOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equ,
  }
  struct TokenIdentifier;
}

pub mod eval {
  use crate::lexer::types::{TokenKeyword, TokenOperator, TokenSeparator};
  pub fn comp_pun(pun: &char) -> Option<TokenSeparator>{
    match pun{
      '(' => Some(TokenSeparator::OpenParent),
      ')' => Some(TokenSeparator::CloseParent),
      '{' => Some(TokenSeparator::LeftCurl),
      '}' => Some(TokenSeparator::RightCurl),
      ';' => Some(TokenSeparator::SemiColon),
      _ => None,
    }
  }
  pub fn is_token_sep(pun: &char) -> bool {
    let value = comp_pun(pun);
    match value {
      None => false,
      _ => true,
    }
  }
  pub fn comp_operator(comp: &char) -> Option<TokenOperator> {
    match comp{
      '+' => Some(TokenOperator::Add),
      '-' => Some(TokenOperator::Sub),
      '/' => Some(TokenOperator::Div),
      '*' => Some(TokenOperator::Mul),
      '=' => Some(TokenOperator::Equ),
      _ => None,
    }
  }
  pub fn is_token_oper(oper: &char) -> bool{
    let value = comp_operator(oper);
    match value {
      None => false,
      _ => true,
    }
  }
  pub fn comp_key(key: &str) -> Option<TokenKeyword> {
    match key{
      "fn"=> Some(TokenKeyword::Function),
      "enum" => Some(TokenKeyword::Enum),
      "struct" => Some(TokenKeyword::Struct),
      _ => None,
    }
  }
}

pub mod tokenizer {
  use crate::lexer::{
    eval::{comp_key, comp_operator, comp_pun, is_token_oper, is_token_sep}, 
    types::{TokenKeyword, LexerCartegories}};

  pub fn tokenize_values(chars: Vec<char>) {
    let mut value: Vec<char> = Vec::new();
    
    for (i, char) in chars.iter().enumerate() {
      if char.is_alphabetic() {
        value.push(*char);
      } else if char.is_whitespace(){
        if value.is_empty(){
          println!("found whitespace")
        } else {
          let string: String = value.drain(..).collect();
          let s = string.as_str();
          match comp_key(s) {
            Some(TokenKeyword::Function) => println!("Function found"),
            Some(TokenKeyword::Enum) => println!("Enum found"),
            Some(TokenKeyword::Struct) => println!("Struct found"),
            None => println!("Unknown / Not done for"),
          }
        }
      } else if is_token_sep(char){
        println!("Found TokenSeparator: {:?}",comp_pun(char));
      } else if is_token_oper(char){
        println!("Found TokenOperator: {:?}", comp_operator(char));
      } else {
        println!("Found smth else: {:?}", char);
      }
    }
  }
}
