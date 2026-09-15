#![allow(unused)]
pub mod types{
  #[derive(Debug,PartialEq,Clone)]
  pub enum LexerCartegories {
    Keyword(TokenKeyword), /* 3rd: PAUSE -- ALMOST DONE */
    Separator(TokenSeparator), /* 1st: PAUSE -- ALMOST DONE*/
    Literal(TokenLiteral), /* LATER ON WHEN NEEDED */
    Operator(TokenOperator), /* 2nd: PAUSE -- ALMOST DONE */
    Identifier(TokenIdentifier),
    EndOfFile,
  }
  
  #[derive(Debug,PartialEq,Clone)]
  pub enum TokenKeyword{
    Function,
    Enum,
    Struct,
  }
  #[derive(Debug,PartialEq,Clone)]
  pub enum TokenSeparator {
    OpenParent,
    CloseParent,
    SemiColon,
    LeftCurl,
    RightCurl,
    Comma,
  }
  #[derive(Debug,PartialEq,Clone)]
  pub enum TokenLiteral {
    True,
    False,
  }
  #[derive(Debug,PartialEq,Clone)]
  pub enum TokenOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equ}
  /*#[derive(Debug)]
  pub enum TokenIdentifier {
    Identify(String)
  }
  #[derive(Debug)]
  struct Identify(pub String);*/
  
  #[derive(Debug,PartialEq,Clone)]
  pub struct TokenIdentifier(String);
  impl TokenIdentifier {
    pub fn new(val: String) -> Self{
      TokenIdentifier(val)
    }
  }
   
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
      ',' => Some(TokenSeparator::Comma),
      _ => None,
    }
  }
  pub fn is_token_sep(pun: &char ) -> bool{
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

pub mod checker {
  use crate::lexer::{
    eval::{comp_key, comp_operator, comp_pun, is_token_oper, is_token_sep}, 
    types::{TokenKeyword, LexerCartegories}};
  pub fn check_values(chars: Vec<char>) {
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
            None => println!("Unknown / Not done for, {}", s),
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

pub mod tokenizer {
  use crate::{lexer::{
    eval::{comp_key, comp_operator, comp_pun, is_token_oper, is_token_sep}, 
    types::{LexerCartegories::{self, EndOfFile}, TokenIdentifier, TokenKeyword, TokenOperator, TokenSeparator}}};
  use owo_colors::OwoColorize;
  use crate::parser::parser::try_val;
  pub fn tokenize(chars: Vec<char>) {
    let mut keyword_value: Vec<char> = Vec::new();
    let mut tokens: Vec<LexerCartegories> = Vec::new(); 
    
    for (i, char) in chars.iter().enumerate() {
      if char.is_alphabetic() || !keyword_value.is_empty() && char.is_numeric(){
        keyword_value.push(*char);
      } else if char.is_whitespace(){ 
        flush_keyword(&mut keyword_value, &mut tokens);
      } else if is_token_sep(char){ 
        flush_keyword(&mut keyword_value, &mut tokens);
        let sep = match char {
          '(' => LexerCartegories::Separator(TokenSeparator::OpenParent),
          ')' => LexerCartegories::Separator(TokenSeparator::CloseParent),
          '{' => LexerCartegories::Separator(TokenSeparator::LeftCurl),
          '}' => LexerCartegories::Separator(TokenSeparator::RightCurl),
          ';' => LexerCartegories::Separator(TokenSeparator::SemiColon),
          ',' => LexerCartegories::Separator(TokenSeparator::Comma),
          _ => unimplemented!(),
        };
        tokens.push(sep);
      } else if is_token_oper(char){
        flush_keyword(&mut keyword_value, &mut tokens);
        let oper = match char {
          '+' => LexerCartegories::Operator(TokenOperator::Add),
          '-' => LexerCartegories::Operator(TokenOperator::Sub),
          '/' => LexerCartegories::Operator(TokenOperator::Div), 
          '*' => LexerCartegories::Operator(TokenOperator::Mul),
          '=' => LexerCartegories::Operator(TokenOperator::Equ),
          _ => unimplemented!(),
        };
        tokens.push(oper);
      } else {
        println!("Found smth else: {:?}", char);
      }
    }
    flush_keyword(&mut keyword_value, &mut tokens);
    tokens.push(EndOfFile);
    println!("\n{:?}",tokens.blue().bold());
    //keyword_type(TokenKeyword::Function,&tokens, 0);
    try_val(&tokens);
    println!("\n{:?}",tokens.blue().bold());
  }
  fn flush_keyword(keyword: &mut Vec<char>, tokens: &mut Vec<LexerCartegories>){
    if keyword.is_empty() {
      return;
    } 
    let val = keyword.drain(..).collect::<String>();
    let value = match comp_key(val.as_str()){
      Some(keyword) => LexerCartegories::Keyword(keyword),
      None => LexerCartegories::Identifier(TokenIdentifier::new(val)),
      //None => LexerCartegories::Identifier(TokenIdentifier::Identify(val.to_string())),
    };
    tokens.push(value);
  }
}