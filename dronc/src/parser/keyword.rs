pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    use crate::parser::errors::parser_error::ParserError;
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftCurl + ... +TokenSeparator::RightCurl
    #[derive(Debug)]
    pub enum KeywordStuff {
        Struct(StructDef),
        Func(FunDef),
        Enum(EnumDef),
    }
    #[derive(Debug)]
    pub struct StructDef {
        pub name: TokenIdentifier,
        pub fields: Option<Vec<TokenIdentifier>>,
    }
    #[derive(Debug)]
    pub struct EnumDef {
        pub name: TokenIdentifier,
        pub fields: Option<Vec<TokenIdentifier>>,
    }
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftBrace + ... + TokenSeparator::RightBrace + TokenSeparator::LeftCurl + ... + TokenSeparator::RightCurl
    #[derive(Debug)]
    pub struct FunDef {
        pub fname: String,
        pub args: Option<Vec<TokenIdentifier>>,
        // Add return logic here for named return.. none named return etc.
        pub body: Option<Vec<LexerCartegories>>,
    }
    impl FunDef {
        pub fn new() -> Self {
            FunDef {
                fname: String::from("main"),
                args: None, //the arguments of the function
                body: None, //the body of the function
            }
        }
    }
    impl Default for FunDef {
        fn default() -> Self {
            Self::new()
        }
    }
}
pub mod parse_keywords {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    use crate::parser::{
        errors::parser_error::ParserError,
        keyword::definition::{EnumDef, FunDef, KeywordStuff, StructDef},
        stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn keyword_parse(
            &mut self,
            token: &TokenKeyword,
        ) -> Result<Vec<KeywordStuff>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut keywordstuffvec: Vec<KeywordStuff> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            match token {
                TokenKeyword::Function => {
                    keywordstuffvec.push(KeywordStuff::Func(self.parse_function()?));
                }
                TokenKeyword::Struct => {
                    keywordstuffvec.push(KeywordStuff::Struct(self.parse_struct()?));
                }
                TokenKeyword::Enum => {
                    keywordstuffvec.push(KeywordStuff::Enum(self.parse_enum()?));
                } //other => return Err(ParserError::UnexpectedToken(other.clone()))),
            }
            Ok(keywordstuffvec)
        }
        pub fn parse_function(&mut self) -> Result<FunDef, ParserError> {
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
        pub fn parse_struct(&mut self) -> Result<StructDef, ParserError> {
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
        pub fn parse_enum(&mut self) -> Result<EnumDef, ParserError> {
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
}
