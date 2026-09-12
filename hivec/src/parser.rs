pub mod parser{
  use lexer::types::LexicalCartegories;
  pub struct Parser {
    tokens: Vec<LexicalCartegories>,
    pos: usize,
  }
  
  impl Parser {
    pub fn new(tokens: Vec<LexicalCartegories>) -> Self {
      Self {tokens,pos: 0}
    }
  }
}