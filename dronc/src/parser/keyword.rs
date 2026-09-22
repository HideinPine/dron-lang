pub mod func {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    pub struct FunDef {
        keyword: TokenKeyword,
        fname: String,
        args: Option<Vec<TokenIdentifier>>,
        body: Option<Vec<LexerCartegories>>,
    }
    impl FunDef {
        pub fn new() -> Self {
            FunDef {
                keyword: TokenKeyword::Function,
                fname: String::from("main"),
                args: None,
                body: None,
            }
        }
        fn add_args(&mut self, args: TokenIdentifier, is_there: bool) {
            if is_there {
                self.args.as_mut().unwrap().push(args)
            }
        }
        fn add_body(&mut self, body: LexerCartegories, is_there: bool) {
            if is_there {
                self.body.as_mut().unwrap().push(body)
            }
        }
    }
    impl Default for FunDef {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[allow(unused)]
pub mod en {}
