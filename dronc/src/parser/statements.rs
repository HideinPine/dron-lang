pub mod definition {
    use crate::lexer::tokens::types::{TokenIdentifier, TokenLiteral};
    use crate::parser::expressions::definition::Expression;
    #[derive(Debug, Clone)]
    pub struct VariableDef {
        pub identifier: TokenIdentifier,
        pub kind: TokenIdentifier,
        pub value: Value,
    }
    #[derive(Debug, Clone)]
    pub struct RetDef {
        pub identifier: Option<Value>,
    }
    #[derive(Debug, Clone)]
    pub enum Value {
        Identifier(TokenIdentifier),
        Literal(TokenLiteral),
        Express(Expression),
    }
    #[derive(Debug)]
    pub enum StateStuff {
        Var(VariableDef),
        Ret(RetDef),
        Cont(()),
        Break(()),
    }
}

pub mod parse_state {
    use crate::lexer::tokens::types::{
        LexerCartegories, TokenAssign, TokenIdentifier, TokenSeparator, TokenState,
    };
    use crate::parser::{
        errors::parser_error::ParserError,
        expressions::definition::Expression,
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
                TokenState::Return => statestuffvec.push(StateStuff::Ret(self.parse_ret()?)),
                TokenState::Continue => statestuffvec.push(StateStuff::Cont(self.parse_cont()?)),
                TokenState::Break => statestuffvec.push(StateStuff::Break(self.parse_break()?)),
                _ => todo!(),
            }
            Ok(statestuffvec)
        }
        pub fn parse_break(&mut self) -> Result<(), ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Break))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(())
        }
        pub fn parse_cont(&mut self) -> Result<(), ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Continue))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(())
        }
        pub fn parse_ret(&mut self) -> Result<RetDef, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Return))?;
            let identifier = match self.temporal() {
                Some(LexerCartegories::Identifier(n)) => Value::Identifier(n.clone()),
                Some(LexerCartegories::Literal(id)) => Value::Literal(id.clone()),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                // TODO => Return above stuff not anything else..
                None => {
                    return Ok(RetDef {
                        identifier: Some(Value::Identifier(TokenIdentifier::new(String::new()))),
                    });
                }
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
            // self.pos += 1;
            let value = match self.peek().cloned() {
                Some(LexerCartegories::Literal(_)) => match self.parse_expression(0) {
                    Ok(Expression::Literal(l)) => {
                        // self.pos += 1;
                        Value::Literal(l)
                    }
                    Ok(Expression::Operation(smth, smth1)) => {
                        Value::Express(Expression::Operation(smth, smth1))
                    }
                    Err(_) => todo!(),
                },
                Some(LexerCartegories::Identifier(id)) => {
                    // self.pos += 1;
                    Value::Identifier(id.clone())
                }
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            // self.pos += 1;
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(VariableDef {
                identifier,
                kind,
                value,
            })
        }
    }
}
