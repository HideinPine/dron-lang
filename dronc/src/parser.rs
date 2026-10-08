//#![allow(unused)]
pub mod block;
pub mod errors;
pub mod keyword;

pub mod stmt {
    use crate::lexer::types::{LexerCartegories, TokenKeyword};
    use crate::parser::{
        block::definition::BlockStuff,
        errors::parser_error::{InvalidTokenError, ParserError},
        keyword::definition::KeywordStuff,
        match_access::function_values,
    };
    #[derive(Debug)]
    pub enum ParseTypes {
        Blocks(BlockStuff),
        Keywords(KeywordStuff),
    }
    pub struct Parser<'a> {
        pub tokens: &'a [LexerCartegories],
        pub pos: usize,
    }
    impl<'a> Parser<'a> {
        pub fn parse_program(&mut self) -> Result<Vec<ParseTypes>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut stuffvec: Vec<ParseTypes> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            while let Some(tok) = self.peek().cloned() {
                match tok {
                    LexerCartegories::EndOfFile => break,
                    LexerCartegories::Keyword(keyword) => {
                        let item = self.keyword_parse(&keyword)?;
                        stuffvec.extend(item.into_iter().map(ParseTypes::Keywords));
                    }
                    LexerCartegories::Block(block) => {
                        let block_value = self.block_parse(&block)?;
                        stuffvec.extend(block_value.into_iter().map(ParseTypes::Blocks));
                    }
                    other => return Err(ParserError::UnexpectedToken(other.clone())),
                };
            }
            Ok(stuffvec)
        }
        pub fn temporal(&mut self) -> Option<&LexerCartegories> {
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
    use crate::parser::errors::keyword_error;
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
        //println!("pos {pos:?}");
        match name {
            Some(LexerCartegories::Identifier(token_identifier)) => Ok(token_identifier.clone()),
            _ => Err(keyword_error::KeywordError::LackIdentifier),
        }
    }
}
