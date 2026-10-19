pub mod types {
    use crate::errors::combined::DronError;
    use crate::lexer::tokens::{
        literal_types::{BoolType, LiteralTypes},
        tokentypes::{TokenIdentifier, TokenLiteral},
    };
    use crate::parser::statements::definition::{Value, VariableDef};
    use crate::semantic::sem_err::type_error::SemanticErrors;

    pub fn type_look_up(var: VariableDef) -> Result<(), DronError> {
        match (&var.kind, &var.value) {
            (LiteralTypes::Int(_), Value::Literal(TokenLiteral::Integer(_))) => Ok(()),
            (LiteralTypes::Float(_), Value::Literal(TokenLiteral::Float(_))) => Ok(()),
            (LiteralTypes::Bool, Value::Literal(TokenLiteral::Boolean(_))) => Ok(()),
            (_, _) => Err(DronError::Semantic(SemanticErrors::TypeMismatch)),
        }
    }
}
pub mod var {
    use std::{collections::HashMap, thread::scope};

    use crate::{
        errors::combined::DronError,
        lexer::tokens::tokentypes::TokenIdentifier,
        parser::{
            keyword::definition::{Block, Body, EnumDef, FunDef, KeywordStuff, StructDef},
            statements::definition::VariableDef,
        },
        semantic::{
            initialize::{NameAble, ScopeStack},
            sem_err::type_error::SemanticErrors,
            variables::types::type_look_up,
        },
    };
    // Function symbol table updater for each new or different function
    pub fn function_stu(
        smth: FunDef,
        mut symbol_table: ScopeStack,
    ) -> Result<ScopeStack, DronError> {
        let name = smth.fname.clone();
        symbol_table.add(name, NameAble::Function);
        for value in smth.body {
            match value {
                Body::Block(_) => {
                    symbol_table.add(TokenIdentifier::new("Block1".to_string()), NameAble::Block);
                }
                Body::Var(var) => {
                    type_look_up(var.clone())?;
                }
                _ => todo!(),
            }
        }
        // scope_checker(smth, &mut symbol_table.clone())?;
        Ok(symbol_table)
    }
}
