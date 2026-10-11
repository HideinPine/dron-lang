#![allow(unused)]
pub mod definition {
    use crate::lexer::tokens::types::{TokenLiteral, TokenOperator};
    pub enum Expression {
        Literal(TokenLiteral),
        Operator(TokenOperator),
    }
}

pub mod parse_express {
    use crate::parser::{
        errors::parser_error::ParserError, expressions::definition::Expression, stmt::Parser,
    };
    impl<'a> Parser<'a> {
        fn parse_expression(&mut self) -> Result<(), ParserError> {
            Ok(())
        }
    }
}
