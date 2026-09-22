#![allow(unused)]
pub mod errors;
pub mod keyword;
pub mod stmt {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    use crate::parser::{keyword::func::FunDef, match_access::function_values};

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
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    use crate::parser::errors::name_error;
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
    fn smth(token_vec: Vec<LexerCartegories>, position: usize, keyword: TokenKeyword) {}
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
        let identifier = TokenIdentifier("".to_string());
        //println!("pos {pos:?}");
        match name {
            Some(LexerCartegories::Identifier(token_identifier)) => {
                println!("Found: {:?}", token_identifier);
            }
            _ => name_error::name_error(name, identifier, pos),
        }
    }
}
