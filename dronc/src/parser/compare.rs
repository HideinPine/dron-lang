#![allow(unused)]
pub mod definition {
    use crate::lexer::tokens::tokentypes::TokenCompare;
    use crate::parser::statements::definition::Value;
    #[derive(Debug)]
    pub struct Comparison {
        pub left: Value,
        pub token: TokenCompare,
        pub right: Value,
    }
}
pub mod parse_compare {
    use crate::parser::{
        compare::definition::Comparison, errors::parser_error::ParserError,
        statements::definition::Value, stmt::Parser,
    };

    use crate::lexer::tokens::tokentypes::{LexerCartegories, TokenCompare};
    impl<'a> Parser<'a> {
        pub fn comparison_parse(&mut self) -> Result<Comparison, ParserError> {
            let left = self.get_value()?;
            match self.peek().cloned() {
                Some(LexerCartegories::Compare(token)) => {
                    self.pos += 1;
                    let right = self.get_value()?;
                    Ok(Comparison { left, token, right })
                }
                Some(other) => return Err(ParserError::UnexpectedToken(other)),
                None => return Err(ParserError::UnexpectedEOF),
            }
        }
    }
}
