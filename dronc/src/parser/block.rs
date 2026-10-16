pub mod definition {
    use crate::lexer::tokens::tokentypes::{LexerCartegories, TokenIdentifier};
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
    pub struct EphReg {
        pub name: Option<TokenIdentifier>,
        pub content: Option<Vec<LexerCartegories>>,
    }

    #[derive(Debug)]
    pub enum BlockStuff {
        Ghost(GhostDef),
        Unsafe(UnsafeDef),
        Ephemeral(EphReg),
    }
}

pub mod parse_blocks {
    use crate::errors::combined::DronError;
    use crate::lexer::tokens::tokentypes::{LexerCartegories, TokenBlock, TokenSeparator};
    use crate::parser::{
        block::definition::{BlockStuff, EphReg, GhostDef, UnsafeDef},
        par_err::parser_error::ParserError,
        stmt::Parser,
    };
    impl<'a> Parser<'a> {
        pub fn block_parse(&mut self, block: &TokenBlock) -> Result<Vec<BlockStuff>, DronError> {
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
                TokenBlock::EphemeralRegion => {
                    blockstuffvec.push(BlockStuff::Ephemeral(self.parse_ephemeral()?));
                }
            }
            Ok(blockstuffvec)
        }
        fn parse_ghost(&mut self) -> Result<GhostDef, DronError> {
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
        fn parse_unsafe(&mut self) -> Result<UnsafeDef, DronError> {
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
        fn parse_ephemeral(&mut self) -> Result<EphReg, DronError> {
            self.expect(LexerCartegories::Block(TokenBlock::EphemeralRegion))?;
            let name = match self.temporal() {
                Some(LexerCartegories::Identifier(n)) => n.clone(),
                Some(tokens) => {
                    return Err(DronError::Parser(ParserError::UnexpectedToken(
                        tokens.clone(),
                    )));
                }
                None => return Err(DronError::Parser(ParserError::UnexpectedEOF)),
            };
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
            Ok(EphReg {
                name: Some(name),
                content: Some(body),
            })
        }
    }
}
