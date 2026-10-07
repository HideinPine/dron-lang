#![allow(unused)]
pub mod block;
pub mod errors;
pub mod keyword;

pub mod stmt {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    use crate::parser::{
        errors::parser_error::{InvalidTokenError, ParserError},
        keyword::definition::FunDef,
        match_access::function_values,
    };

    pub struct Parser<'a> {
        pub tokens: &'a [LexerCartegories],
        pub pos: usize,
    }
    impl<'a> Parser<'a> {
        pub fn next(&mut self) -> Option<&LexerCartegories> {
            //todo!()
            let t = self.tokens.get(self.pos);
            self.pos += 1;
            t
        }
        pub fn peek(&self) -> Option<&LexerCartegories> {
            self.tokens.get(self.pos)
        }
        pub fn expect(&mut self, expected: LexerCartegories) -> Result<(), ParserError> {
            match self.peek() {
                Some(val) if *val == expected => {
                    self.pos += 1;
                    Ok(())
                }
                Some(found) => Err(ParserError::InvalidToken(InvalidTokenError {
                    expected,
                    found: found.clone(),
                })),
                None => Err(ParserError::UnexpectedEOF),
            }
        }
    }
    pub enum Def {
        Func(FunDef),
    }

    pub fn try_val(token_vec: &[LexerCartegories]) {
        for (i, token) in token_vec.iter().enumerate() {
            if *token == LexerCartegories::EndOfFile {
                /*token_vec.push(FunDef::new());*/
                //println!("EndOfFile reached\n");
                break;
            } else if *token == LexerCartegories::Keyword(TokenKeyword::Function) {
                match function_values(token_vec, i) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {e}"),
                }
            } else {
                continue;
            }
        }
    }
}

pub mod match_access {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword};
    use crate::parser::errors::{keyword_error, name_error};
    use crate::parser::stmt::Parser;

    fn keyword_type(
        keyword: TokenKeyword,
        token_vec: Vec<LexerCartegories>,
        position: usize,
    ) -> Result<TokenIdentifier, keyword_error::KeywordError> {
        let pos = position + 1;
        //println!("postion: {position} found: {keyword:?}");
        match keyword {
            TokenKeyword::Enum => get_values(&token_vec, pos),
            TokenKeyword::Function => get_values(&token_vec, pos),
            TokenKeyword::Struct => get_values(&token_vec, pos),
        }
    }
    //fn smth(token_vec: Vec<LexerCartegories>, position: usize, keyword: TokenKeyword) {}
    pub fn function_values(
        token_vec: &[LexerCartegories],
        position: usize,
    ) -> Result<TokenIdentifier, keyword_error::KeywordError> {
        keyword_type(TokenKeyword::Function, token_vec.to_vec(), position)
    }
    fn get_values(
        token_vec: &[LexerCartegories],
        position: usize,
    ) -> Result<TokenIdentifier, keyword_error::KeywordError> {
        let parser = Parser {
            tokens: token_vec,
            pos: position,
        };
        let name = parser.peek();
        let pos = parser.pos;
        let identifier = TokenIdentifier("".to_string());
        //println!("pos {pos:?}");
        match name {
            Some(LexerCartegories::Identifier(token_identifier)) => Ok(token_identifier.clone()),
            _ => Err(keyword_error::KeywordError::LackIdentifier),
            //_ => name_error::name_error(name, identifier, pos),
        }
    }
}
