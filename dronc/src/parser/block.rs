pub mod definition {
    use crate::lexer::types::{LexerCartegories, TokenBlock, TokenSeparator};
    // TokenBlock + TokenSeparator(OpenCurl) + ... + TokenSeparator(CloseCurl)
    #[derive(Debug)]
    pub struct GhostDef {
        pub content: Option<Vec<LexerCartegories>>,
    }
    pub enum BlockStuff {
        Ghost(GhostDef),
    }
}

pub mod parse_blocks {
    use crate::lexer::types::{LexerCartegories, TokenBlock, TokenIdentifier, TokenSeparator};
    use crate::parser::{
        block::definition::{BlockStuff, GhostDef},
        errors::parser_error::ParserError,
        stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn blocks_parse(&mut self) -> Result<Vec<BlockStuff>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut blockstuffvec: Vec<BlockStuff> = Vec::new();
            // if let Some(LexerCartegories::Block(TokenBlock::Ghost)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            while let Some(tok) = self.peek() {
                match tok {
                    LexerCartegories::EndOfFile => break,
                    LexerCartegories::Block(TokenBlock::Ghost) => {
                        blockstuffvec.push(BlockStuff::Ghost(self.parse_ghost()?));
                    }
                    other => return Err(ParserError::UnexpectedToken(other.clone())),
                }
            }
            Ok(blockstuffvec)
        }
        fn parse_ghost(&mut self) -> Result<GhostDef, ParserError> {
            self.expect(LexerCartegories::Block(TokenBlock::Ghost))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(name)) => name.clone(),
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEOF),
            };
            self.expect(LexerCartegories::Separator(TokenSeparator::LeftCurl))?;
            let mut body: Vec<LexerCartegories> = Vec::new();
            while let Some(field) = self.peek() {
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
    }
}
