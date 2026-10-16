pub mod type_error {
    #[derive(Debug)]
    pub enum SemanticErrors {
        TypeMismatch,
        UndeclaredVar,
        OutOfScopeVar,
        MultipleDeclarationVar,
        ReservedVar,
    }
    impl std::error::Error for SemanticErrors {}
    impl std::fmt::Display for SemanticErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SemanticErrors::MultipleDeclarationVar => {
                    write!(f, "Variable declared multiple times")
                }
                SemanticErrors::OutOfScopeVar => write!(f, "Variable out of scope"),
                SemanticErrors::TypeMismatch => write!(f, "Type mismatch"),
                SemanticErrors::UndeclaredVar => write!(f, "Use of undeclared variable"),
                SemanticErrors::ReservedVar => write!(f, "Use of reserved variable"),
            }
        }
    }
}
