pub mod sem_err;
mod variables;

pub mod initialize {
    use crate::{
        errors::combined::DronError,
        parser::{
            block::{self, definition::BlockStuff},
            keyword::definition::KeywordStuff,
            statements::definition::StateStuff,
            stmt::AbstractSyntaxTree,
        },
        semantic::{sem_err::type_error::SemanticErrors, variables::types::type_look_up},
    };

    pub fn init(ast: AbstractSyntaxTree) -> Result<(), DronError> {
        for state in &ast.states {
            match state {
                StateStuff::Var(smth) => type_look_up(smth.clone())?,
                _ => return Err(DronError::Semantic(SemanticErrors::TypeMismatch)),
            }
        }
        // for keyword in &ast.keywords {
        //     match keyword {
        //         KeywordStuff::Struct(_smth) => todo!(),
        //         KeywordStuff::Enum(_) => todo!(),
        //         KeywordStuff::Func(_) => todo!(),
        //     }
        // }
        // for blocks in &ast.blocks {
        //     match blocks {
        //         BlockStuff::Ephemeral(_) => todo!(),
        //         BlockStuff::Ghost(_) => todo!(),
        //         BlockStuff::Unsafe(_) => todo!(),
        //     }
        // }
        Ok(())
    }
}
