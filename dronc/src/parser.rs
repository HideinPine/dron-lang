#![allow(unused)]
pub mod block;
pub mod errors;
pub mod keyword;

pub mod stmt {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    use crate::parser::{
        errors::parser_error::{InvalidTokenError, ParserError},
        keyword::definition::{EnumDef, FunDef, KeywordStuff, StructDef},
        match_access::function_values,
    };

    pub struct Parser<'a> {
        pub tokens: &'a [LexerCartegories],
        pub pos: usize,
    }
    impl<'a> Parser<'a> {
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
    impl<'a> Parser<'a> {
        fn parse_function(&mut self) -> Result<FunDef, ParserError> {
            self.expect(LexerCartegories::Keyword(TokenKeyword::Function))?;
            let fname = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.0.clone(),
                Some(tokens) => return Err(ParserError::UnexpectedToken(tokens.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftBrace))?;
            let mut args = Vec::new();
            while let Some(LexerCartegories::Identifier(arg)) = self.peek() {
                args.push(arg.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightBrace))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let body = Vec::new();
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(FunDef {
                fname,
                args: Some(args),
                body: Some(body),
            })
        }
        pub fn parse_program(&mut self) -> Result<Vec<KeywordStuff>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut keywordstuffvec: Vec<KeywordStuff> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            while let Some(tok) = self.peek() {
                match tok {
                    LexerCartegories::EndOfFile => break,
                    LexerCartegories::Keyword(TokenKeyword::Function) => {
                        keywordstuffvec.push(KeywordStuff::Func(self.parse_function()?));
                    }
                    LexerCartegories::Keyword(TokenKeyword::Struct) => {
                        keywordstuffvec.push(KeywordStuff::Struct(self.parse_struct()?));
                    }
                    LexerCartegories::Keyword(TokenKeyword::Enum) => {
                        keywordstuffvec.push(KeywordStuff::Enum(self.parse_enum()?));
                    }
                    other => return Err(ParserError::UnexpectedToken(other.clone())),
                }
            }
            Ok(keywordstuffvec)
        }
        fn parse_struct(&mut self) -> Result<StructDef, ParserError> {
            self.expect(LexerCartegories::Keyword(TokenKeyword::Struct))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut body: Vec<TokenIdentifier> = Vec::new();
            while let Some(LexerCartegories::Identifier(field)) = self.peek() {
                body.push(field.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(StructDef {
                name,
                fields: Some(body),
            })
        }
        fn parse_enum(&mut self) -> Result<EnumDef, ParserError> {
            self.expect(LexerCartegories::Keyword(TokenKeyword::Enum))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut body: Vec<TokenIdentifier> = Vec::new();
            while let Some(LexerCartegories::Identifier(arg)) = self.peek() {
                body.push(arg.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(EnumDef {
                name,
                fields: Some(body),
            })
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
