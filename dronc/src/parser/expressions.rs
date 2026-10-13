#![allow(unused)]
pub mod definition {
    use crate::lexer::tokens::types::{TokenLiteral, TokenOperator};
    pub enum Express {
        Literal(TokenLiteral),
        Operator(TokenOperator),
    }
    pub struct Expression {
        pub value: Vec<Express>,
    }
}

pub mod parse_express {
    use crate::lexer::tokens::types::{LexerCartegories, TokenLiteral};
    use crate::parser::{
        errors::parser_error::ParserError,
        expressions::definition::{Express, Expression},
        stmt::Parser,
    };
    // impl<'a> Parser<'a> {
    //     pub fn parse_expression(&mut self, influence: u8) -> Result<Expression, ParserError> {
    //         let mut value: Vec<Express> = Vec::new();
    //         Ok(Expression { value })
    //     }
    //     fn parse_lit(&self) -> Result<
    // }
}
