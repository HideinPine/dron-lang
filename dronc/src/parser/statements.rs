pub mod definition {
    use crate::lexer::tokens::types::{TokenIdentifier, TokenLiteral};
    #[derive(Debug)]
    pub struct VariableDef {
        pub identifier: TokenIdentifier,
        pub kind: TokenIdentifier,
        pub value: TokenLiteral,
    }
    #[derive(Debug)]
    pub enum StateStuff {
        Var(VariableDef),
    }
}

pub mod parse_state {
    use crate::lexer::tokens::types::{LexerCartegories, TokenAssign, TokenSeparator, TokenState};
    use crate::parser::{
        errors::parser_error::ParserError,
        statements::definition::{StateStuff, VariableDef},
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
                _ => todo!(),
            }
            Ok(statestuffvec)
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
                Some(LexerCartegories::Literal(id)) => id.clone(),
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
