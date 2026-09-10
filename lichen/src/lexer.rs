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