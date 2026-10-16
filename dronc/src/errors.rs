pub mod combined {
    use crate::errors::combined::DronError::Semantic;
    use crate::parser::par_err::{
        keyword_error::KeywordError, lexer_error::LexerError, parser_error::ParserError,
    };
    use crate::semantic::sem_err::type_error::SemanticErrors;

    #[derive(Debug)]
    pub enum DronError {
        Semantic(SemanticErrors),
        Lexer(LexerError),
        Parser(ParserError),
        Keyword(KeywordError),
    }
    impl From<SemanticErrors> for DronError {
        fn from(err: SemanticErrors) -> Self {
            DronError::Semantic(err)
        }
    }
    impl From<LexerError> for DronError {
        fn from(err: LexerError) -> Self {
            DronError::Lexer(err)
        }
    }
    impl From<ParserError> for DronError {
        fn from(err: ParserError) -> Self {
            DronError::Parser(err)
        }
    }
    impl From<KeywordError> for DronError {
        fn from(err: KeywordError) -> Self {
            DronError::Keyword(err)
        }
    }
    impl std::fmt::Display for DronError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                DronError::Semantic(err) => write!(f, "Semantic analysis error: {:?}", err),
                DronError::Lexer(err) => write!(f, "Lexer Error: {:?}", err),
                DronError::Parser(err) => write!(f, "Parser Error: {:?}", err),
                DronError::Keyword(err) => write!(f, "Keyword error: {:?}", err),
            }
        }
    }
    impl std::error::Error for DronError {}
}
