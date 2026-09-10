#![allow(unused)]
pub mod types{
  pub struct LexerCartegories {
    keyword: TokenLiteral,
    separator: TokenSeparator, /* First to do coz it's simpler */
    literal: TokenLiteral,
    operator: TokenOperator,
  }
  enum TokenKeyword{
    Function,
    Enum,
    Struct,
  }
  pub enum TokenSeparator {
    OpenParent,
    CloseParent,
    SemiColon,
    LeftCurl,
    RightCurl,
    Unknown,
  }
  enum TokenLiteral {
    True,
    False,
  }
  enum TokenOperator {
    Add,
    Sub,
    Mul,
    Div,
  }
}
pub mod checker {
  use crate::lexer::types::LexerCartegories;

  /* Determine the next plausible value and then match to the one needed */
  fn get_nxt_value(chars: Vec<char>, i: usize)  {
    let char_value = chars[i];
    let mut value = Vec::new();
    value.push(char_value);
    println!("{:?}", value);
  }

  pub fn check_values(chars: Vec<char>) {
    for (i, char) in chars.iter().enumerate() {
      if char.is_numeric() {
        //get_nxt_value(chars.clone(), i);
        println!("Digit character: {}, {}", char, i);
      } else if char.is_whitespace() {
        println!("Whitespace character: {}", char);
      } else if char.is_alphabetic() {
        println!("Alphabetic character: {}", char);
      } else if char.is_ascii_punctuation(){
        match char {
          _ => println!("Match stuff not done for char: {}",char),
        }
      }
    }
  }
}

pub mod parser {
    
}