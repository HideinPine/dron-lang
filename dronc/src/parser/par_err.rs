pub mod lexer_error {
    #[derive(Debug)]
    pub enum LexerError {
        FloatError,
        InvalidDigitError(String),
        NotYetImplemented,
    }
    impl std::error::Error for LexerError {}
    impl std::fmt::Display for LexerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LexerError::FloatError => write!(f, "Too many points.. not a valid decimal"),
                LexerError::InvalidDigitError(e) => write!(f, "Invalid digit: {:?}", e),
                LexerError::NotYetImplemented => write!(f, "Not yet implemented"),
            }
        }
    }
}
pub mod parser_error {
    use crate::lexer::tokens::tokentypes::LexerCartegories;
    #[derive(Debug)]
    pub enum ParserError {
        UnexpectedToken(LexerCartegories),
        UnexpectedEOF,
        NotYetImplemented,
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
                ParserError::NotYetImplemented => write!(f, "Not yet implemented"),
                ParserError::InvalidToken(err) => write!(
                    f,
                    "Invalid token: expected {:?} found {:?}",
                    err.expected, err.found
                ),
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
