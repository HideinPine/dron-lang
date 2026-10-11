pub mod definition {
    use crate::lexer::tokens::types::{Arguments, TokenIdentifier};
    use crate::parser::statements::definition::VariableDef;
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
        pub fields: Vec<Arguments>,
    }
    #[derive(Debug)]
    pub struct EnumDef {
        pub name: TokenIdentifier,
        pub fields: Vec<TokenIdentifier>,
    }
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftBrace + ... + TokenSeparator::RightBrace + TokenSeparator::LeftCurl + ... + TokenSeparator::RightCurl
    #[derive(Debug, Clone)]
    pub struct FunDef {
        pub fname: String,
        pub args: Vec<Arguments>,
        // Add return logic here for named return.. none named return etc.
        pub body: Vec<Body>,
    }
    #[derive(Debug, Clone)]
    pub struct Block(Box<Vec<Body>>);
    impl Block {
        pub fn new(body: Vec<Body>){
            Block(Box::new(body));
        }
    }
    #[derive(Debug, Clone)]
    pub enum Body {
        Func(FunDef),
        Var(VariableDef),
        Block(Block),
    }
}
pub mod parse_keywords {
    use crate::lexer::tokens::types::{
        Arguments, LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator, TokenState,
    };
    use crate::parser::{
        errors::parser_error::ParserError,
        keyword::definition::{Body, EnumDef, Block, FunDef, KeywordStuff, StructDef},
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
            let mut depth = 0;
            self.expect(LexerCartegories::Keyword(TokenKeyword::Function))?;
            let fname = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.0.clone(),
                Some(tokens) => return Err(ParserError::UnexpectedToken(tokens.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftBrace))?;
            let mut args: Vec<Arguments> = Vec::new();
            while let Some(LexerCartegories::Identifier(name)) = self.peek().cloned() {
                self.pos += 1;
                self.expect(LexerCartegories::Separator(TokenSeparator::Colon))?;
                let arg_type = match self.temporal() {
                    Some(LexerCartegories::Identifier(t)) => t.clone(),
                    Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                    None => return Err(ParserError::UnexpectedEOF),
                };
                args.push(Arguments { name, arg_type });
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightBrace))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            depth += 1;
            let mut body: Vec<Body> = Vec::new();
            let block_body: Vec<Body> = Vec::new();
            let value = Block::new(block_body.clone());
            while depth > 0 {
                match self.peek().cloned() {
                       Some(LexerCartegories::Separator(TokenSeparator::LeftCurl)) => {
                           depth += 1;
                           //Block logic starts here tbh.
                           let _block = value;
                           self.pos += 1;
                       }
                       Some(LexerCartegories::Separator(TokenSeparator::RightCurl)) => {
                           depth -= 1;
                           self.pos += 1;
                       }
                       Some(LexerCartegories::Keyword(TokenKeyword::Function)) => {
                           body.push(Body::Func(self.parse_function()?));
                       }
                       Some(LexerCartegories::Statement(TokenState::VariableDeclaration)) => {
                           body.push(Body::Var(self.parse_variable()?));
                       }

                       Some(other) => return Err(ParserError::UnexpectedToken(other)),
                       None => return Err(ParserError::UnexpectedEOF),
                   }
            }
            Ok(FunDef { fname, args, body })
        }
        pub fn parse_struct(&mut self) -> Result<StructDef, ParserError> {
            self.expect(LexerCartegories::Keyword(TokenKeyword::Struct))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut fields: Vec<Arguments> = Vec::new();
            while let Some(LexerCartegories::Identifier(name)) = self.peek().cloned() {
                self.pos += 1;
                self.expect(LexerCartegories::Separator(TokenSeparator::Colon))?;
                let arg_type = match self.temporal() {
                    Some(LexerCartegories::Identifier(t)) => t.clone(),
                    Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                    None => return Err(ParserError::UnexpectedEOF),
                };
                fields.push(Arguments { name, arg_type });
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(StructDef { name, fields })
        }
        pub fn parse_enum(&mut self) -> Result<EnumDef, ParserError> {
            self.expect(LexerCartegories::Keyword(TokenKeyword::Enum))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut fields: Vec<TokenIdentifier> = Vec::new();
            while let Some(LexerCartegories::Identifier(arg)) = self.peek() {
                fields.push(arg.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(EnumDef { name, fields })
        }
    }
}
