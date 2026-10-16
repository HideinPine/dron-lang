pub mod types {
    use crate::lexer::tokens::{
        literal_types::{BoolType, LiteralTypes},
        tokentypes::TokenLiteral,
    };
    use crate::parser::statements::definition::{Value, VariableDef};
    use crate::semantic::sem_err::type_error::SemanticErrors;

    pub fn type_look_up(var: VariableDef) -> Result<(), SemanticErrors> {
        match var.kind {
            LiteralTypes::Int(_) => match var.value {
                Value::Literal(TokenLiteral::Integer(_)) => Ok(()),
                _ => Err(SemanticErrors::TypeMismatch),
            },
            LiteralTypes::Float(_) => match var.value {
                Value::Literal(TokenLiteral::Float(_)) => Ok(()),
                _ => Err(SemanticErrors::TypeMismatch),
            },
        }
    }
}
