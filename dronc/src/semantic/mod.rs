pub mod sem_err;
mod variables;
pub mod initialize {
    use crate::{
        errors::combined::DronError,
        lexer::tokens::tokentypes::TokenIdentifier,
        parser::{
            block::{self, definition::BlockStuff},
            keyword::definition::{Body, KeywordStuff},
            statements::definition::StateStuff,
            stmt::AbstractSyntaxTree,
        },
        semantic::{
            sem_err::type_error::SemanticErrors,
            variables::{types::type_look_up, var::function_stu},
        },
    };
    use owo_colors::OwoColorize;
    use std::{collections::HashMap, hash::Hash, thread::scope};
    #[derive(Debug, Clone)]
    pub enum NameAble {
        Struct,
        Enum,
        Function,
        Variable,
        Block,
    }
    #[derive(Debug, Clone)]
    pub struct ScopeStack {
        scopes: Vec<HashMap<TokenIdentifier, NameAble>>,
    }
    impl ScopeStack {
        fn new() -> Self {
            ScopeStack {
                scopes: vec![HashMap::new()],
            }
        }

        pub fn add(&mut self, key: TokenIdentifier, kind: NameAble) -> Result<(), DronError> {
            if let Some(value) = self.scopes.last_mut().unwrap().get(&key) {
                return Err(DronError::Semantic(SemanticErrors::ReservedVar));
            }
            self.scopes.last_mut().unwrap().insert(key, kind);
            Ok(())
        }
    }
    pub fn init(ast: AbstractSyntaxTree) -> Result<(), DronError> {
        let mut symbol_table = ScopeStack::new();
        for state in ast.states {
            match state {
                StateStuff::Var(smth) => {
                    type_look_up(smth.clone())?;
                    symbol_table.add(smth.identifier, NameAble::Variable);
                }
                _ => return Err(DronError::Semantic(SemanticErrors::TypeMismatch)),
            }
        }
        for keyword in ast.keywords {
            match keyword {
                KeywordStuff::Struct(smth) => {
                    symbol_table.add(smth.name, NameAble::Struct);
                }
                KeywordStuff::Enum(smth) => {
                    symbol_table.add(smth.name, NameAble::Enum);
                }
                KeywordStuff::Func(smth) => {
                    let value = function_stu(smth, symbol_table.clone())?;
                    // symbol_table.append(value);
                }
            };
        }
        println!("{:#?}", symbol_table.yellow().bold());
        Ok(())
    }
}
