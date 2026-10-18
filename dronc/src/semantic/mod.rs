pub mod sem_err;
mod variables;

pub mod initialize {
    use crate::{
        errors::combined::DronError,
        lexer::tokens::tokentypes::TokenIdentifier,
        parser::{
            block::{self, definition::BlockStuff},
            keyword::definition::KeywordStuff,
            statements::definition::StateStuff,
            stmt::AbstractSyntaxTree,
        },
        semantic::{
            sem_err::type_error::SemanticErrors,
            variables::{types::type_look_up, var::scope_checker},
        },
    };
    use owo_colors::OwoColorize;
    use std::collections::HashMap;
    #[derive(Debug)]
    pub enum NameAble {
        Struct,
        Enum,
        Function,
        Variable,
    }
    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum BlockNames {
        Global,
        Function,
        InlineFunction,
    }
    pub fn init(ast: AbstractSyntaxTree) -> Result<(), DronError> {
        let mut symbol_table: Vec<HashMap<BlockNames ,HashMap<TokenIdentifier, NameAble>>> = vec![HashMap::new()];
        let mut secnd_hash: Vec<HashMap<TokenIdentifier, NameAble>> = vec![HashMap::new()]; 
        for state in ast.states {
            match state {
                StateStuff::Var(smth) => {
                    type_look_up(smth.clone())?;
                    symbol_table
                        .last_mut()
                        .unwrap()
                        .entry(BlockNames::Global)
                        .or_insert_with(HashMap::new)
                        .insert(smth.identifier, NameAble::Variable);
                }
                _ => return Err(DronError::Semantic(SemanticErrors::TypeMismatch)),
            }
        }
        for keyword in ast.keywords {
            match keyword {
                KeywordStuff::Struct(smth) => {
                    symbol_table
                        .last_mut()
                        .unwrap()
                        .entry(BlockNames::Global)
                        .or_insert_with(HashMap::new)
                        .insert(smth.name, NameAble::Struct);
                }
                KeywordStuff::Enum(smth) => {
                    symbol_table
                        .last_mut()
                        .unwrap()
                        .entry(BlockNames::Global)
                        .or_insert_with(HashMap::new)
                        .insert(smth.name, NameAble::Enum);
                }
                KeywordStuff::Func(smth) => {
                    let name = smth.fname.clone();
                    symbol_table
                        .last_mut()
                        .unwrap()
                        .entry(BlockNames::Global)
                        .or_insert_with(HashMap::new)
                        .insert(name, NameAble::Function);
                    let mut value = scope_checker(smth)?;
                    symbol_table.append(&mut value);
                }
            };
        }
        println!("{:#?}", symbol_table.yellow().bold());
        Ok(())
    }
}
