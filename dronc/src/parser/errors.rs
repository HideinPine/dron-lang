pub mod name_error {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier};
    use owo_colors::OwoColorize;
    pub fn name_error(value: Option<&LexerCartegories>, expected: TokenIdentifier, pos: usize) {
        println!(
            "{}{:?} expected {:?}{}{}",
            "Error Found: ".bold().red(),
            value.unwrap().bold().red(),
            expected.bold().red(),
            " at position: ".red().bold(),
            pos.bold()
        );
        exit();
    }
    #[allow(unused)]
    fn exit() {}
}
pub mod parser_error {
    use crate::lexer::types::LexerCartegories;
    #[derive(Debug)]
    pub enum ParserError {
        UnexpectedToken(LexerCartegories),
        UnexpectedEOF,
        InvalidToken(InvalidTokenError),
    }
    #[derive(Debug)]
    pub struct InvalidTokenError {
        pub expected: LexerCartegories,
        pub found: LexerCartegories,
    }
    impl std::fmt::Display for ParserError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ParserError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
                ParserError::UnexpectedEOF => write!(f, "Unexpected end of file"),
                ParserError::InvalidToken(err) => write!(f, "Invalid token: {:?}", err),
            }
        }
    }
    impl std::error::Error for ParserError {}
}

pub mod keyword_error {
    use owo_colors::OwoColorize;
    #[derive(Debug)]
    pub enum KeywordError {
        UnknownKeyword(String),
        LackIdentifier,
        LackKeyword,
    }
    impl std::fmt::Display for KeywordError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                KeywordError::UnknownKeyword(keyword) => {
                    write!(f, "Unknown keyword: {}", keyword.bold().red())
                }
                KeywordError::LackIdentifier => write!(f, "Lack identifier"),
                KeywordError::LackKeyword => write!(f, "Lack keyword"),
            }
        }
    }
    impl std::error::Error for KeywordError {}
}
