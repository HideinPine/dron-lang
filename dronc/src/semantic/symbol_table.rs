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
pub mod table_def {
    use crate::{errors::combined::DronError, semantic::sem_err::type_error::SemanticErrors};
    use indexmap::{IndexMap, IndexSet};

    #[derive(Debug, Clone)]
    pub enum SymbolTypes {
        Struct,
        Enum,
        Function,
        Variable,
        Constant,
        Block,
    }
    #[derive(Debug)]
    pub struct SymbolTable {
        scopes: Vec<IndexMap<String, SymbolTypes>>,
    }

    impl SymbolTable {
        pub fn new() -> Self {
            Self {
                scopes: vec![IndexMap::new()],
            }
        }
        pub fn rem(&mut self) {
            self.scopes.pop();
        }
        pub fn add(&mut self) {
            self.scopes.push(IndexMap::new());
        }
        pub fn lookup(&self, name: &str) -> Result<(), DronError> {
            match self.scopes.iter().rev().find_map(|f| f.get(name)) {
                Some(_) => Err(DronError::Semantic(SemanticErrors::ReservedVar)),
                None => Ok(()),
            }
        }
        pub fn declare(&mut self, name: String, ty: SymbolTypes) -> Result<(), DronError> {
            let scope = self
                .scopes
                .last_mut()
                .expect("The GlobalScope Always Exists");
            if scope.contains_key(name.as_str()) {
                return Err(DronError::Semantic(SemanticErrors::MultipleDeclarationVar));
            }
            scope.insert(name, ty);
            Ok(())
        }
    }
}
pub mod table_analyser {
    use crate::{
        errors::combined::DronError,
        lexer::tokens::tokentypes::LexerCartegories::Keyword,
        parser::{
            block::definition::BlockStuff,
            keyword::definition::KeywordStuff,
            statements::definition::{StateStuff, Value},
            stmt::AbstractSyntaxTree,
        },
        semantic::{
            key_sem::keywords::{extract_enum, extract_func, extract_struct},
            symbol_table::table_def::{SymbolTable, SymbolTypes},
        },
    };
    use indexmap::IndexMap;

    pub fn analyse_ast(ast: AbstractSyntaxTree) -> Result<SymbolTable, DronError> {
        let mut symbol_table = SymbolTable::new();
        for i in ast.keywords.iter() {
            match i {
                KeywordStuff::Enum(enu) => extract_enum(enu, &mut symbol_table)?,
                KeywordStuff::Func(fun) => extract_func(fun, &mut symbol_table)?,
                KeywordStuff::Struct(stru) => extract_struct(stru, &mut symbol_table)?,
            }
        }
        for i in ast.blocks.iter() {
            match i {
                BlockStuff::Unsafe(_) => todo!(),
                BlockStuff::Ephemeral(_) => todo!(),
                BlockStuff::Ghost(_) => todo!(),
            }
        }
        for i in ast.states.iter() {
            match i {
                _ => todo!(),
            }
        }
        Ok(symbol_table)
    }
}
