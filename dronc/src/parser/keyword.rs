pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier, TokenKeyword, TokenSeparator};
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftCurl + ... +TokenSeparator::RightCurl
    #[repr(C)]
    pub struct Keyword {
        keyword: TokenKeyword,
        name: TokenIdentifier,
        open_curl: TokenSeparator,
        fields: Option<Vec<TokenIdentifier>>,
        close_curl: TokenSeparator,
    }
    impl Keyword {
        pub fn template(keyword: TokenKeyword, name: TokenIdentifier) -> Self {
            Self {
                keyword,
                name,
                open_curl: TokenSeparator::LeftCurl,
                fields: None,
                close_curl: TokenSeparator::RightCurl,
            }
        }
    }
    // TokenKeyword + TokenIdentifier + TokenSeparator::LeftBrace + ... + TokenSeparator::RightBrace + TokenSeparator::LeftCurl + ... + TokenSeparator::RightCurl
    #[repr(C)]
    pub struct FunDef {
        keyword: TokenKeyword,
        fname: String,
        open_brace: TokenSeparator,
        args: Option<Vec<TokenIdentifier>>,
        close_brace: TokenSeparator,
        //== todo ==//
        // Add return logic here for named return.. none named return etc.
        open_curl: TokenSeparator,
        body: Option<Vec<LexerCartegories>>,
        close_curl: TokenSeparator,
    }
    impl FunDef {
        pub fn new() -> Self {
            FunDef {
                keyword: TokenKeyword::Function,
                fname: String::from("main"),
                open_brace: TokenSeparator::LeftBrace,
                args: None, //the arguments of the function
                close_brace: TokenSeparator::RightBrace,
                open_curl: TokenSeparator::LeftCurl,
                body: None, //the body of the function
                close_curl: TokenSeparator::RightCurl,
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

#[allow(unused)]
pub mod en {}
