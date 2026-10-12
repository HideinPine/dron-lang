pub mod definition {
    use crate::lexer::tokens::types::{Arguments, TokenIdentifier};
    use crate::parser::statements::definition::{RetDef, VariableDef};
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
    #[allow(unused)]
    #[derive(Debug, Clone)]
    pub struct Block(Box<Vec<Body>>);
    impl Block {
        pub fn new(body: Vec<Body>) -> Self {
            Block(Box::new(body))
        }
    }
    #[derive(Debug, Clone)]
    pub enum Body {
        Func(FunDef),
        Var(VariableDef),
        Block(Block),
        Return(RetDef),
        // Continue(Cont)
    }
}
pub mod parse_keywords {
    use crate::lexer::tokens::types::{
        Arguments, LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator, TokenState,
    };
    use crate::parser::{
        errors::parser_error::ParserError,
        keyword::definition::{Block, Body, EnumDef, FunDef, KeywordStuff, StructDef},
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
            // let mut depth = 0;
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
            // depth += 1;let mut stack: Vec<Vec<Body>> = vec![Vec::new()];   // the function's body
            let mut stack: Vec<Vec<Body>> = vec![Vec::new()];
            while let Some(token) = self.peek().cloned() {
                match token {
                    LexerCartegories::Separator(TokenSeparator::LeftCurl) => {
                        stack.push(Vec::new());
                        self.pos += 1;
                    }
                    LexerCartegories::Separator(TokenSeparator::RightCurl) => {
                        self.pos += 1;
                        if stack.len() == 1 {
                            break;
                        }
                        let inner = stack.pop().unwrap();
                        stack
                            .last_mut()
                            .unwrap()
                            .push(Body::Block(Block::new(inner)));
                    }
                    LexerCartegories::Keyword(TokenKeyword::Function) => {
                        let value = self.parse_function()?;
                        stack.last_mut().unwrap().push(Body::Func(value));
                    }
                    LexerCartegories::Statement(TokenState::VariableDeclaration) => {
                        let value = self.parse_variable()?;
                        stack.last_mut().unwrap().push(Body::Var(value));
                    }
                    LexerCartegories::Statement(TokenState::Return) => {
                        let value = self.parse_ret()?;
                        stack.last_mut().unwrap().push(Body::Return(value));
                    }
                    other => return Err(ParserError::UnexpectedToken(other)),
                }
            }
            let body = stack.pop().unwrap();
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
