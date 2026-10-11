pub mod definition {
    use crate::lexer::tokens::types::{TokenIdentifier, TokenLiteral};
    #[derive(Debug, Clone)]
    pub struct VariableDef {
        pub identifier: TokenIdentifier,
        pub kind: TokenIdentifier,
        pub value: Value,
    }
    #[derive(Debug)]
    pub struct RetDef {
        pub identifier: Option<Value>,
    }
    #[derive(Debug, Clone)]
    pub enum Value {
        Identifier(TokenIdentifier),
        Literal(TokenLiteral),
    }
    #[derive(Debug)]
    pub enum StateStuff {
        Var(VariableDef),
        Ret(RetDef),
        Continue,
        Break,
    }
}

pub mod parse_state {
    use crate::lexer::tokens::types::{
        LexerCartegories, TokenAssign, TokenIdentifier, TokenSeparator, TokenState,
    };
    use crate::parser::{
        errors::parser_error::ParserError,
        statements::definition::{RetDef, StateStuff, Value, VariableDef},
        stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn statement_parse(
            &mut self,
            token: &TokenState,
        ) -> Result<Vec<StateStuff>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut statestuffvec: Vec<StateStuff> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            match token {
                TokenState::VariableDeclaration => {
                    statestuffvec.push(StateStuff::Var(self.parse_variable()?))
                }
                TokenState::Return => {
                    statestuffvec.push(StateStuff::Ret(self.parse_ret()?))
                }
                _ => todo!(),
            }
            Ok(statestuffvec)
        }
        pub fn parse_ret(&mut self) -> Result<RetDef, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Return))?;
            let identifier = match self.temporal() {
                Some(LexerCartegories::Identifier(n)) => Value::Identifier(n.clone()),
                Some(LexerCartegories::Literal(id)) => Value::Literal(id.clone()),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                // TODO => Return above stuff not anything else..
                None => return Ok(RetDef{identifier: Some(Value::Identifier(TokenIdentifier::new(String::new())))}),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(RetDef {
                identifier: Some(identifier),
            })
        }
        pub fn parse_variable(&mut self) -> Result<VariableDef, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::VariableDeclaration))?;
            let identifier = match self.temporal() {
                Some(LexerCartegories::Identifier(n)) => n.clone(),
                Some(tokens) => return Err(ParserError::UnexpectedToken(tokens.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::Colon))?;
            let kind = match self.temporal() {
                Some(LexerCartegories::Identifier(literal)) => literal.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Assign(TokenAssign::Equal))?;
            let value = match self.temporal() {
                Some(LexerCartegories::Literal(id)) => Value::Literal(id.clone()),
                Some(LexerCartegories::Identifier(id)) => Value::Identifier(id.clone()),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(VariableDef {
                identifier,
                kind,
                value,
            })
        }
    }
}
