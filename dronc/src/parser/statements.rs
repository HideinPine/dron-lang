pub mod definition {
    use crate::lexer::tokens::{
        literal_types::LiteralTypes,
        tokentypes::{TokenIdentifier, TokenLiteral},
    };
    use crate::parser::{compare::definition::Comparison, expressions::definition::Expression};
    #[derive(Debug, Clone)]
    pub struct VariableDef {
        pub identifier: TokenIdentifier,
        pub kind: LiteralTypes,
        pub value: Value,
    }
    #[derive(Debug, Clone)]
    pub struct RetDef {
        pub id: Option<Value>,
    }
    #[derive(Debug, Clone)]
    pub enum Value {
        Identifier(TokenIdentifier),
        Literal(TokenLiteral),
        Express(Expression),
    }
    #[derive(Debug)]
    pub enum Contains {
        Var(VariableDef),
        Expression(Expression),
        Return(RetDef),
    }
    #[derive(Debug)]
    pub struct IfStuff {
        pub expression: Comparison,
        pub contains: Vec<Contains>,
    }
    #[derive(Debug)]
    pub struct ElseIfStuff {
        pub expression: Comparison,
        pub contains: Vec<Contains>,
    }
    #[derive(Debug)]
    pub struct ElseStuff {
        pub contains: Vec<Contains>,
    }
    #[derive(Debug)]
    pub struct WhileStuff {
        pub comparison: Comparison,
        pub contains: Vec<StateStuff>,
    }
    #[derive(Debug)]
    pub struct LoopStuff {
        pub contains: Vec<StateStuff>,
    }
    #[derive(Debug)]
    pub enum StateStuff {
        Var(VariableDef),
        Ret(RetDef),
        If(IfStuff),
        ElseIf(ElseIfStuff),
        Else(ElseStuff),
        Loop(LoopStuff),
        While(WhileStuff),
        Cont(()),
        Break(()),
    }
}

pub mod parse_state {
    use crate::lexer::tokens::tokentypes::{
        LexerCartegories, TokenAssign, TokenSeparator, TokenState,
    };
    use crate::parser::{
        errors::parser_error::ParserError,
        expressions::definition::Expression,
        statements::definition::{
            Contains, ElseIfStuff, ElseStuff, IfStuff, LoopStuff, RetDef, StateStuff, Value,
            VariableDef, WhileStuff,
        },
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
                TokenState::If => statestuffvec.push(StateStuff::If(self.parse_if()?)),
                TokenState::ElseIf => statestuffvec.push(StateStuff::ElseIf(self.parse_elseif()?)),
                TokenState::Else => statestuffvec.push(StateStuff::Else(self.parse_else()?)),
                TokenState::Loop => statestuffvec.push(StateStuff::Loop(self.parse_loop()?)),
                TokenState::While => statestuffvec.push(StateStuff::While(self.parse_while()?)),
                _ => return Err(ParserError::NotYetImplemented),
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
            let identifier = match self.peek().cloned() {
                Some(LexerCartegories::Identifier(n)) => {
                    self.pos += 1;
                    Value::Identifier(n.clone())
                }
                Some(LexerCartegories::Literal(_)) => match self.parse_expression(0) {
                    Ok(Expression::Literal(l)) => Value::Literal(l),
                    Ok(Expression::Identifier(l)) => Value::Identifier(l),
                    Ok(Expression::Operation(smth, smth1)) => {
                        Value::Express(Expression::Operation(smth, smth1))
                    }
                    Err(e) => return Err(e),
                },
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                // TODO => Return above stuff not anything else..
                None => {
                    return Ok(RetDef { id: None });
                }
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::SemiColon))?;
            Ok(RetDef {
                id: Some(identifier),
            })
        }
        pub fn parse_elseif(&mut self) -> Result<ElseIfStuff, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::ElseIf))?;
            let expression = self.comparison_parse()?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut stack: Vec<Vec<Contains>> = vec![Vec::new()];
            while let Some(token) = self.peek().cloned() {
                match token {
                    LexerCartegories::Separator(TokenSeparator::RightCurl) => break,

                    LexerCartegories::Statement(TokenState::VariableDeclaration) => {
                        let value = self.parse_variable()?;
                        stack.last_mut().unwrap().push(Contains::Var(value));
                    }
                    LexerCartegories::Statement(TokenState::Return) => {
                        let value = self.parse_ret()?;
                        stack.last_mut().unwrap().push(Contains::Return(value));
                    }
                    other => return Err(ParserError::UnexpectedToken(other)),
                }
            }
            let contains = stack.pop().unwrap();
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(ElseIfStuff {
                expression,
                contains,
            })
        }
        pub fn parse_if(&mut self) -> Result<IfStuff, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::If))?;
            let expression = self.comparison_parse()?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut stack: Vec<Vec<Contains>> = vec![Vec::new()];
            while let Some(token) = self.peek().cloned() {
                match token {
                    LexerCartegories::Separator(TokenSeparator::RightCurl) => break,

                    LexerCartegories::Statement(TokenState::VariableDeclaration) => {
                        let value = self.parse_variable()?;
                        stack.last_mut().unwrap().push(Contains::Var(value));
                    }
                    LexerCartegories::Statement(TokenState::Return) => {
                        let value = self.parse_ret()?;
                        stack.last_mut().unwrap().push(Contains::Return(value));
                    }
                    other => return Err(ParserError::UnexpectedToken(other)),
                }
            }
            let contains = stack.pop().unwrap();
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(IfStuff {
                expression,
                contains,
            })
        }
        pub fn parse_else(&mut self) -> Result<ElseStuff, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Else))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut stack: Vec<Vec<Contains>> = vec![Vec::new()];
            while let Some(token) = self.peek().cloned() {
                match token {
                    LexerCartegories::Separator(TokenSeparator::RightCurl) => break,

                    LexerCartegories::Statement(TokenState::VariableDeclaration) => {
                        let value = self.parse_variable()?;
                        stack.last_mut().unwrap().push(Contains::Var(value));
                    }
                    LexerCartegories::Statement(TokenState::Return) => {
                        let value = self.parse_ret()?;
                        stack.last_mut().unwrap().push(Contains::Return(value));
                    }
                    other => return Err(ParserError::UnexpectedToken(other)),
                }
            }
            let contains = stack.pop().unwrap();
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(ElseStuff { contains })
        }
        pub fn parse_loop(&mut self) -> Result<LoopStuff, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::Loop))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let contains = self.get_loop()?;
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(LoopStuff { contains })
        }
        pub fn parse_while(&mut self) -> Result<WhileStuff, ParserError> {
            self.expect(LexerCartegories::Statement(TokenState::While))?;
            let comparison = self.comparison_parse()?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let contains = self.get_loop()?;
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(WhileStuff {
                comparison,
                contains,
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
                Some(LexerCartegories::LitType(ty)) => ty.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Assign(TokenAssign::Equal))?;
            // self.pos += 1;
            let value = self.get_value()?;
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
