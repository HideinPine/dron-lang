pub mod sem_err;
mod typecheck;

pub mod initialize {
    use crate::parser::{
        block::definition::BlockStuff, keyword, statements::definition::StateStuff,
        stmt::AbstractSyntaxTree,
    };
    use crate::semantic::{sem_err::type_error::SemanticErrors, typecheck::types::type_look_up};

    pub fn init(ast: AbstractSyntaxTree) -> Result<(), SemanticErrors> {
        for state in &ast.states {
            match state {
                StateStuff::Var(smth) => type_look_up(smth.clone())?,
                _ => return Err(SemanticErrors::TypeMismatch),
            }
        }
        Ok(())
    }
}
