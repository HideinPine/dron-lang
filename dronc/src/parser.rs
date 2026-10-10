//#![allow(unused)]
pub mod block;
pub mod errors;
pub mod keyword;

pub mod stmt {
    use crate::lexer::tokens::types::LexerCartegories;
    use crate::parser::{
        block::definition::BlockStuff,
        errors::parser_error::{InvalidTokenError, ParserError},
        keyword::definition::KeywordStuff,
    };
    use owo_colors::OwoColorize;
    #[derive(Debug)]
    pub enum ParseTypes {
        Blocks(BlockStuff),
        Keywords(KeywordStuff),
    }
    pub struct Parser<'a> {
        pub tokens: &'a [LexerCartegories],
        pub pos: usize,
    }
    impl<'a> Parser<'a> {
        pub fn parse_program(&mut self) -> Result<Vec<ParseTypes>, ParserError> {
            //let mut funcvec: Vec<FunDef> = Vec::new();
            let mut stuffvec: Vec<ParseTypes> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            while let Some(tok) = self.peek().cloned() {
                match tok {
                    LexerCartegories::EndOfFile => break,
                    LexerCartegories::Keyword(keyword) => {
                        let item = self.keyword_parse(&keyword)?;
                        stuffvec.extend(item.into_iter().map(ParseTypes::Keywords));
                    }
                    LexerCartegories::Block(block) => {
                        let block_value = self.block_parse(&block)?;
                        stuffvec.extend(block_value.into_iter().map(ParseTypes::Blocks));
                    }
                    other => return Err(ParserError::UnexpectedToken(other.clone())),
                };
            }
            Ok(stuffvec)
        }
        pub fn temporal(&mut self) -> Option<&LexerCartegories> {
            //todo!()
            let t = self.tokens.get(self.pos);
            self.pos += 1;
            t
        }
        pub fn peek(&self) -> Option<&LexerCartegories> {
            self.tokens.get(self.pos)
        }
        pub fn expect(&mut self, expected: LexerCartegories) -> Result<(), ParserError> {
            match self.peek() {
                Some(val) if *val == expected => {
                    self.pos += 1;
                    Ok(())
                }
                Some(found) => Err(ParserError::InvalidToken(InvalidTokenError {
                    expected,
                    found: found.clone(),
                })),
                None => Err(ParserError::UnexpectedEOF),
            }
        }
    }

    pub fn get_tokens(tokens: Vec<LexerCartegories>) {
        let mut parser = Parser {
            tokens: &tokens,
            pos: 0,
        };
        match parser.parse_program() {
            Ok(smth) => println!("\n{:?}", smth.green().bold()),
            Err(er) => println!("{:?}", er),
        }
        println!("\n{:?}", tokens.blue().bold());
        //println!("Total lines are: {newline_counter}\n");
        //try_val(&tokens);
    }
}
