//#![allow(unused)]
pub mod block;
pub mod compare;
pub mod errors;
pub mod expressions;
pub mod keyword;
pub mod statements;

pub mod stmt {
    use crate::lexer::tokens::tokentypes::{LexerCartegories, TokenSeparator, TokenState};
    use crate::parser::{
        block::definition::BlockStuff,
        errors::parser_error::{InvalidTokenError, ParserError},
        expressions::definition::Expression,
        keyword::definition::KeywordStuff,
        statements::definition::{StateStuff, Value},
    };
    use owo_colors::OwoColorize;
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct AbstractSyntaxTree {
        blocks: Vec<BlockStuff>,
        keywords: Vec<KeywordStuff>,
        states: Vec<StateStuff>,
    }
    pub struct Parser<'a> {
        pub tokens: &'a [LexerCartegories],
        pub pos: usize,
    }
    impl<'a> Parser<'a> {
        pub fn parse_program(&mut self) -> Result<AbstractSyntaxTree, ParserError> {
            let mut keywords_vec: Vec<KeywordStuff> = Vec::new();
            let mut blocks_vec: Vec<BlockStuff> = Vec::new();
            let mut state_vec: Vec<StateStuff> = Vec::new();
            // if let Some(LexerCartegories::Keyword(TokenKeyword::Struct)) = self.peek() {
            //     //TODO: Solve for repetitive code here later
            while let Some(tok) = self.peek().cloned() {
                match tok {
                    LexerCartegories::EndOfFile => break,
                    LexerCartegories::Keyword(keyword) => {
                        let item = self.keyword_parse(&keyword)?;
                        keywords_vec.extend(item);
                    }
                    LexerCartegories::Block(block) => {
                        let block_value = self.block_parse(&block)?;
                        blocks_vec.extend(block_value);
                        // stuffvec.extend(block_value.into_iter().map(AbstractSyntaxTree::Blocks));
                    }
                    LexerCartegories::Statement(state) => {
                        let item = self.statement_parse(&state)?;
                        state_vec.extend(item);
                    }
                    other => {
                        return Err(ParserError::UnexpectedToken(other.clone()));
                    }
                };
            }
            Ok(AbstractSyntaxTree {
                blocks: blocks_vec,
                keywords: keywords_vec,
                states: state_vec,
            })
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
        pub fn get_value(&mut self) -> Result<Value, ParserError> {
            match self.peek().cloned() {
                Some(LexerCartegories::Literal(_)) => match self.parse_expression(0) {
                    Ok(Expression::Literal(l)) => {
                        // self.pos += 1;
                        Ok(Value::Literal(l))
                    }
                    Ok(Expression::Identifier(smth)) => Ok(Value::Identifier(smth)),
                    Ok(Expression::Operation(smth, smth1)) => {
                        Ok(Value::Express(Expression::Operation(smth, smth1)))
                    }
                    Err(e) => Err(e),
                },
                Some(LexerCartegories::Identifier(id)) => {
                    self.pos += 1;
                    Ok(Value::Identifier(id.clone()))
                }
                #[allow(clippy::needless_return)]
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => Err(ParserError::UnexpectedEOF),
            }
        }
        pub fn get_loop(&mut self) -> Result<Vec<StateStuff>, ParserError> {
            let mut stack: Vec<Vec<StateStuff>> = vec![Vec::new()];
            while let Some(token) = self.peek().cloned() {
                match token {
                    LexerCartegories::Separator(TokenSeparator::RightCurl) => break,
                    LexerCartegories::Statement(TokenState::VariableDeclaration) => {
                        let value = self.parse_variable()?;
                        stack.last_mut().unwrap().push(StateStuff::Var(value));
                    }
                    LexerCartegories::Statement(TokenState::Break) => {
                        self.parse_break()?;
                        stack.last_mut().unwrap().push(StateStuff::Break(()));
                    }
                    LexerCartegories::Statement(TokenState::Continue) => {
                        self.parse_cont()?;
                        stack.last_mut().unwrap().push(StateStuff::Cont(()));
                    }
                    LexerCartegories::Statement(TokenState::Return) => {
                        let value = self.parse_ret()?;
                        stack.last_mut().unwrap().push(StateStuff::Ret(value));
                    }
                    LexerCartegories::Statement(TokenState::Loop) => {
                        let value = self.parse_loop()?;
                        stack.last_mut().unwrap().push(StateStuff::Loop(value));
                    }
                    LexerCartegories::Statement(TokenState::If) => {
                        let value = self.parse_if()?;
                        stack.last_mut().unwrap().push(StateStuff::If(value));
                    }
                    LexerCartegories::Statement(TokenState::ElseIf) => {
                        let value = self.parse_elseif()?;
                        stack.last_mut().unwrap().push(StateStuff::ElseIf(value));
                    }
                    LexerCartegories::Statement(TokenState::Else) => {
                        let value = self.parse_else()?;
                        stack.last_mut().unwrap().push(StateStuff::Else(value));
                    }
                    LexerCartegories::Statement(TokenState::While) => {
                        let value = self.parse_while()?;
                        stack.last_mut().unwrap().push(StateStuff::While(value));
                    }
                    other => return Err(ParserError::UnexpectedToken(other)),
                }
            }
            let contains = stack.pop().unwrap();
            Ok(contains)
        }
    }
    pub fn get_tokens(tokens: Vec<LexerCartegories>) {
        let mut parser = Parser {
            tokens: &tokens,
            pos: 0,
        };
        match parser.parse_program() {
            // Ok(_) =>
            Ok(smth) => println!(
                "\n {} {:?}",
                "THE AST:".blue().bold(),
                smth.bright_green().bold()
            ),
            Err(er) => println!("{} {:?}", "Error:".red().bold(), er.red().bold()),
        }
        println!("\n{:?}", tokens.blue().bold());
        //println!("Total lines are: {newline_counter}\n");
        //try_val(&tokens);
    }
}
