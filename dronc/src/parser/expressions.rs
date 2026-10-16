#![allow(unused)]
pub mod definition {
    use crate::lexer::tokens::tokentypes::{TokenIdentifier, TokenLiteral, TokenOperator};

    #[derive(Debug, Clone)]
    pub enum Expression {
        Identifier(TokenIdentifier),
        Literal(TokenLiteral),
        Operation(TokenOperator, Vec<Expression>),
    }
}

pub mod parse_express {
    use crate::lexer::tokens::tokentypes::{LexerCartegories, TokenLiteral, TokenOperator};
    use crate::parser::{
        expressions::definition::Expression, par_err::parser_error::ParserError,
        statements::definition::Value, stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, ParserError> {
            let mut left = match self.temporal() {
                Some(LexerCartegories::Literal(smth)) => Expression::Literal(smth.clone()),
                Some(LexerCartegories::Identifier(smth)) => Expression::Identifier(smth.clone()),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            loop {
                let op = match self.peek() {
                    Some(LexerCartegories::EndOfFile) => break,
                    Some(LexerCartegories::Operator(op)) => op.clone(),
                    Some(_) => break,
                    None => return Err(ParserError::UnexpectedEOF),
                };
                let (left_bind, right_bind) = op.precedence();
                if left_bind < min_precedence {
                    break;
                }
                self.temporal();
                let right = self.parse_expression(right_bind)?;
                left = Expression::Operation(op.clone(), vec![left, right])
            }
            Ok(left)
        }
    }
}
