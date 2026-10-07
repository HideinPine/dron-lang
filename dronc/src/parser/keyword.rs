pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftCurl + ... +TokenSeparator::RightCurl
    pub struct Keyword {
        keyword: TokenKeyword,
        name: TokenIdentifier,
        fields: Option<Vec<TokenIdentifier>>,
    }
    impl Keyword {
        pub fn template(keyword: TokenKeyword, name: TokenIdentifier) -> Self {
            Self {
                keyword,
                name,
                fields: None,
            }
        }
    }
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftBrace + ... + TokenSeparator::RightBrace + TokenSeparator::LeftCurl + ... + TokenSeparator::RightCurl
    pub struct FunDef {
        keyword: TokenKeyword,
        fname: String,
        args: Option<Vec<TokenIdentifier>>,
        // Add return logic here for named return.. none named return etc.
        body: Option<Vec<LexerCartegories>>,
    }
    impl FunDef {
        pub fn new() -> Self {
            FunDef {
                keyword: TokenKeyword::Function,
                fname: String::from("main"),
                args: None, //the arguments of the function
                body: None, //the body of the function
            }
        }
        fn add_args(&mut self, arguments: TokenIdentifier) {
            if let Some(args) = &mut self.args {
                args.push(arguments)
            }
        }
        fn add_body(&mut self, values: LexerCartegories) {
            if let Some(body) = &mut self.body {
                body.push(values)
            }
        }
    }
    impl Default for FunDef {
        fn default() -> Self {
            Self::new()
        }
    }
}
