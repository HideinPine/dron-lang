#![allow(unused)]
pub mod functione;
pub mod stmt {
    use crate::lexer::types::{LexerCartegories, TokenKeyword, TokenIdentifier};
    use crate::parser::match_access::function_values;

    pub struct Parser {
        pub vector: Vec<LexerCartegories>,
        pub pos: usize,
    }
    impl Parser {
        pub fn consume(&self) -> LexerCartegories {
            <LexerCartegories as Clone>::clone(&self.vector[self.pos])
        }
        pub fn peek(&self) -> Option<&LexerCartegories> {
            self.vector.get(self.pos)
        }
        pub fn iseof(&self) -> bool {
            self.vector[self.pos] == LexerCartegories::EndOfFile
        }
    }
    pub enum Def {
        Func(FunDef),
    }
    pub struct FunDef {
        fname: String,
        args: Option<Vec<TokenIdentifier>>,
        body: Option<Vec<LexerCartegories>>,
    }
    impl FunDef {
        fn new() -> Self {
            FunDef {
                fname: String::from("main"),
                args: None,
                body: None,
            }
        }
        fn add_args(&mut self, args: TokenIdentifier, is_there: bool) {
          if is_there == true { self.args.as_mut().unwrap().push(args) } else {return;}
        }
        fn add_body(&mut self, body: LexerCartegories, is_there: bool) {
          if is_there == true { self.body.as_mut().unwrap().push(body) } else {return}
        }
        
    }
    pub fn try_val(token_vec: &[LexerCartegories]) {
        for (i, token) in token_vec.iter().enumerate() {
            if *token == LexerCartegories::EndOfFile {
                /*token_vec.push(FunDef::new());*/
                println!("EndOfFile reached\n");
                break;
            } else if *token == LexerCartegories::Keyword(TokenKeyword::Function) {
                function_values(token_vec, i);
            } else {
                continue;
            }
        }
    }
}

pub mod match_access {
    use crate::functione::name_error;
    use crate::lexer::types::{LexerCartegories, TokenKeyword};
    use crate::parser::stmt::Parser;

    fn keyword_type(keyword: TokenKeyword, token_vec: Vec<LexerCartegories>, position: usize) {
        let pos = position + 1;
        //println!("postion: {position} found: {keyword:?}");
        match keyword {
            TokenKeyword::Enum => get_values(token_vec, pos),
            TokenKeyword::Function => get_values(token_vec, pos),
            TokenKeyword::Struct => get_values(token_vec, pos),
        }
    }
    pub fn function_values(token_vec: &[LexerCartegories], position: usize) {
        keyword_type(TokenKeyword::Function, token_vec.to_vec(), position);
    }
    fn get_values(token_vec: Vec<LexerCartegories>, position: usize) {
        let parser = Parser {
            vector: token_vec,
            pos: position,
        };
        let name = parser.peek();
        let pos = parser.pos;
        //println!("pos {pos:?}");
        match name {
            Some(LexerCartegories::Identifier(token_identifier)) => {
                println!("Found: {:?}", token_identifier);
            }
            _ => name_error::function_name_error(name, pos),
        }
    }
}
