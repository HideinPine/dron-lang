pub mod types {
    use crate::lexer::tokens::{
        literal_types::{BoolType, LiteralTypes},
        tokentypes::TokenLiteral,
    };
    use crate::parser::statements::definition::{Value, VariableDef};
    use crate::semantic::sem_err::type_error::SemanticErrors;

    pub fn type_look_up(var: VariableDef) -> Result<(), SemanticErrors> {
        match (&var.kind, &var.value) {
            (LiteralTypes::Int(_), Value::Literal(TokenLiteral::Integer(_))) => Ok(()),
            (LiteralTypes::Float(_), Value::Literal(TokenLiteral::Float(_))) => Ok(()),
            (LiteralTypes::Bool, Value::Literal(TokenLiteral::Boolean(_))) => Ok(()),
            (_, _) => Err(SemanticErrors::TypeMismatch),
        }
    }
}
