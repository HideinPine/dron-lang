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
            initialize::{BlockNames, NameAble},
            sem_err::type_error::SemanticErrors,
            variables::types::type_look_up,
        },
    };
    pub fn scope_checker(
        function: FunDef,
    ) -> Result<Vec<HashMap<BlockNames, HashMap<TokenIdentifier, NameAble>>>, DronError> {
        let mut symb: Vec<HashMap<BlockNames, HashMap<TokenIdentifier, NameAble>>> =
            vec![HashMap::new()];
        let fn_clone = function.clone();
        for value in fn_clone.body {
            match value {
                Body::Block(_) => {
                    symb.push(HashMap::new());
                    scope_checker(function.clone())?;
                },
                
                _ => todo!(),
            }
        }
        Ok(symb)
    }
}
