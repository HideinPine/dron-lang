pub mod parser{
  use lexer::types::{LexicalCartegories, TokenSeparator};
  
  pub struct Parser {
    tokens: Vec<LexicalCartegories>,
    pos: usize,
  }
  impl Parser {
    pub fn new(tokens: Vec<LexicalCartegories>) -> Self {
      Self {tokens,pos: 0}
    }
  }
  
  pub enum Def{
    Func(FunDef),
  }
  pub struct FunDef {
    fname: String,
    args: Option<Vec<LexicalCartegories>>,
    body: Vec<LexicalCartegories>,
  }
}