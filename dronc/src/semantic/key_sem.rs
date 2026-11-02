//extract meaning
pub mod keywords {
    use crate::{
        errors::combined::DronError,
        parser::{
            keyword::definition::{Block, Body, EnumDef, FunDef, StructDef},
            stmt::AbstractSyntaxTree,
        },
        semantic::{
            key_sem::statements::extract_vars,
            sem_err::type_error::SemanticErrors,
            symbol_table::table_def::{Function, SymbolTable, SymbolTypes},
        },
    };
    use indexmap::IndexMap;

    pub fn extract_func(fun: &FunDef, sym: &mut SymbolTable) -> Result<(), DronError> {
        sym.lookup(&fun.fname.0.clone())?;
        //exit that scope and start a go back to the old one..

        extract_body(&fun.body, sym)?;
        sym.declare(fun.fname.0.clone(), SymbolTypes::Function(Function::new()))?;
        Ok(())
    }
    pub fn extract_body(body: &[Body], sym: &mut SymbolTable) -> Result<Function, DronError> {
        // sym.add();
        for i in body {
            match i {
                Body::Var(v) => extract_vars(v, sym)?,
                Body::Block(b) => todo!(),
                // Body::Block(block) => extract_block(block.clone(), sym)?,
                Body::Func(f) => extract_func(f, sym)?,
                Body::Expression(_) | Body::Return(_) => {}
            }
        }
        // sym.rem();
        Ok(Function::new())
    }
    fn extract_block(body: Block, sym: &mut SymbolTable) -> Result<Function, DronError> {
        let storage: IndexMap<String, SymbolTypes> = IndexMap::new();
        for i in body.iter() {
            match i {
                Body::Var(v) => {
                    // if sym.lookup(&v.identifier.0) == Err(_) {}
                    extract_vars(v, sym)?
                }
                _ => todo!(),
            }
        }
        extract_body(&body, sym)
    }
    pub fn extract_enum(en: &EnumDef, sym: &mut SymbolTable) -> Result<(), DronError> {
        sym.lookup(&en.name.0.clone())?;
        sym.declare(en.name.0.clone(), SymbolTypes::Enum)?;
        Ok(())
    }
    pub fn extract_struct(stru: &StructDef, sym: &mut SymbolTable) -> Result<(), DronError> {
        sym.lookup(&stru.name.0.clone())?;
        sym.declare(stru.name.0.clone(), SymbolTypes::Struct)?;
        Ok(())
    }
}
pub mod statements {
    use crate::{
        errors::combined::DronError,
        parser::statements::definition::VariableDef,
        semantic::symbol_table::table_def::{SymbolTable, SymbolTypes},
    };
    pub fn extract_vars(var: &VariableDef, sym: &mut SymbolTable) -> Result<(), DronError> {
        let name = var.identifier.0.clone();
        sym.lookup(&name)?;
        sym.declare(name, SymbolTypes::Variable)?;
        Ok(())
    }
}
