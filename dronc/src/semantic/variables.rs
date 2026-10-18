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
    use crate::{
        errors::combined::DronError,
        parser::{
            keyword::definition::{EnumDef, FunDef, StructDef},
            statements::definition::VariableDef,
        },
    };
    #[derive(Debug)]
    enum IdStuff {
        Fun(FunDef),
        Struct(StructDef),
        Enum(EnumDef),
        Var(VariableDef),
    }
    pub fn identifiers() -> Result<(), DronError> {
        Ok(())
    }
}
