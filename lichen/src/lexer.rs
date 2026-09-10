pub mod types{
  #![allow(unused)]
  pub struct LexerCartegories {
    keyword: TokenLiteral,
    separator: TokenSeparator,
    literal: TokenLiteral,
    operator: TokenOperator,
  }
  enum TokenKeyword{
    Function,
    Enum,
    Struct,
  }
  enum TokenSeparator {
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
  enum TokenOperator {
    Add,
    Sub,
    Mul,
    Div,
  }
}
pub mod checker {
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
        get_nxt_value(chars.clone(), i);
        println!("Digit character: {}, {}", char, i);
      } else if char.is_whitespace() {
        println!("Whitespace character: {}", char);
      } else if char.is_alphabetic() {
        println!("Alphabetic character: {}", char);
      } else{
        println!("Other character: {}", char);
      }
    }
  }
}

pub mod parser {
    
}