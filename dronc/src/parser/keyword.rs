pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    use crate::parser::errors::parser_error::ParserError;
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftCurl + ... +TokenSeparator::RightCurl
    #[derive(Debug)]
    pub enum KeywordStuff {
        Struct(StructDef),
        Func(FunDef),
        Enum(EnumDef),
    }
    #[derive(Debug)]
    pub struct StructDef {
        pub name: TokenIdentifier,
        pub fields: Option<Vec<TokenIdentifier>>,
    }
    #[derive(Debug)]
    pub struct EnumDef {
        pub name: TokenIdentifier,
        pub fields: Option<Vec<TokenIdentifier>>,
    }
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftBrace + ... + TokenSeparator::RightBrace + TokenSeparator::LeftCurl + ... + TokenSeparator::RightCurl
    #[derive(Debug)]
    pub struct FunDef {
        pub fname: String,
        pub args: Option<Vec<TokenIdentifier>>,
        // Add return logic here for named return.. none named return etc.
        pub body: Option<Vec<LexerCartegories>>,
    }
    impl FunDef {
        pub fn new() -> Self {
            FunDef {
                fname: String::from("main"),
                args: None, //the arguments of the function
                body: None, //the body of the function
            }
        }
    }
    impl Default for FunDef {
        fn default() -> Self {
            Self::new()
        }
    }
}
