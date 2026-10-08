pub mod definition {
    use crate::lexer::types::LexerCartegories;
    // TokenBlock + TokenSeparator(OpenCurl) + ... + TokenSeparator(CloseCurl)
    #[derive(Debug)]
    pub struct GhostDef {
        pub content: Option<Vec<LexerCartegories>>,
    }
    #[derive(Debug)]
    pub struct UnsafeDef {
        pub content: Option<Vec<LexerCartegories>>,
    }
    #[derive(Debug)]
    pub enum BlockStuff {
        Ghost(GhostDef),
        Unsafe(UnsafeDef),
    }
}

pub mod parse_blocks {
    use crate::lexer::types::{LexerCartegories, TokenBlock, TokenSeparator};
    use crate::parser::{
        block::definition::{BlockStuff, GhostDef, UnsafeDef},
        errors::parser_error::ParserError,
        stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn block_parse(&mut self, block: &TokenBlock) -> Result<Vec<BlockStuff>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut blockstuffvec: Vec<BlockStuff> = Vec::new();
            // if let Some(LexerCartegories::Block(TokenBlock::Ghost)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            match block {
                TokenBlock::Ghost => {
                    blockstuffvec.push(BlockStuff::Ghost(self.parse_ghost()?));
                }
                TokenBlock::Unsafe => {
                    blockstuffvec.push(BlockStuff::Unsafe(self.parse_unsafe()?));
                }
            }
            Ok(blockstuffvec)
        }
        fn parse_ghost(&mut self) -> Result<GhostDef, ParserError> {
            self.expect(LexerCartegories::Block(TokenBlock::Ghost))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut body: Vec<LexerCartegories> = Vec::new();
            while let Some(field) = self.peek() {
                if let LexerCartegories::Separator(TokenSeparator::RightCurl) = field {
                    break;
                }
                body.push(field.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(GhostDef {
                content: Some(body),
            })
        }
        fn parse_unsafe(&mut self) -> Result<UnsafeDef, ParserError> {
            self.expect(LexerCartegories::Block(TokenBlock::Unsafe))?;
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut body: Vec<LexerCartegories> = Vec::new();
            while let Some(field) = self.peek() {
                if let LexerCartegories::Separator(TokenSeparator::RightCurl) = field {
                    break;
                }
                body.push(field.clone());
                self.pos += 1;
                if let Some(LexerCartegories::Separator(TokenSeparator::Comma)) = self.peek() {
                    self.pos += 1;
                }
            }
            self.expect(LexerCartegories::Separator(TokenSeparator::RightCurl))?;
            Ok(UnsafeDef {
                content: Some(body),
            })
        }
    }
}
