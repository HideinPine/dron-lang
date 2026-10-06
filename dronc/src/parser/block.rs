pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenBlock, TokenSeparator};
    // TokenBlock + TokenSeparator(OpenCurl) + ... + TokenSeparator(CloseCurl)
    pub struct Block {
        block: TokenBlock,
        open_curl: TokenSeparator,
        content: Option<Vec<LexerCartegories>>,
        close_curl: TokenSeparator,
    }

    impl Block {
        pub fn block_template(block: TokenBlock, content: Option<Vec<LexerCartegories>>) -> Self {
            Self {
                block,
                open_curl: TokenSeparator::LeftCurl,
                content,
                close_curl: TokenSeparator::RightCurl,
            }
        }
    }
}

pub mod block_types {}
