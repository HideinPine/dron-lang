pub mod tokentypes {
    #[allow(unused)]
    use crate::lexer::tokens::literal_types::{BoolType, FloatType, IntType, LiteralTypes};
    #[derive(Debug, PartialEq, Clone)]
    pub enum LexerCartegories {
        Keyword(TokenKeyword), /* 3rd: PAUSE -- ALMOST DONE */
        Block(TokenBlock),
        Separator(TokenSeparator), /* 1st: PAUSE -- ALMOST DONE*/
        Literal(TokenLiteral),     /* LATER ON WHEN NEEDED */
        Statement(TokenState),
        Operator(TokenOperator), /* 2nd: PAUSE -- ALMOST DONE */
        Identifier(TokenIdentifier),
        Compare(TokenCompare),
        Assign(TokenAssign),
        LitType(LiteralTypes),
        EndOfFile,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenKeyword {
        Function,
        Enum,
        Struct,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenBlock {
        Ghost, //ghost block
        Unsafe,
        EphemeralRegion,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenSeparator {
        LeftBrace,
        RightBrace,
        SemiColon,
        LeftCurl,
        RightCurl,
        Comma,
        Colon,
        /* TODO: Dot, Colon, */
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenAssign {
        Equal,
        SingleArrow, // -> i.e return operator.
        DoubleArrow, // =>
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenLiteral {
        Boolean(BoolType),
        Integer(i64),
        Float(f64),
        String,
        Char,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenOperator {
        Add,
        Sub,
        Mul,
        Div,
    }
    impl TokenOperator {
        pub fn precedence(&self) -> (u8, u8) {
            match self {
                TokenOperator::Add | TokenOperator::Sub => (2, 3),
                TokenOperator::Mul | TokenOperator::Div => (3, 4),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenCompare {
        DoubleEqual,
        Greater,
        Less,
        NotEqual,
        GreaterEqual,
        LessEqual,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct TokenIdentifier(pub String);
    impl TokenIdentifier {
        pub fn new(val: String) -> Self {
            TokenIdentifier(val)
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenState {
        VariableDeclaration,
        If,
        Else,
        ElseIf,
        While,
        For,
        Loop,
        Return,
        Continue,
        Break,
    }
    #[derive(Debug, Clone)]
    pub struct Arguments {
        pub name: TokenIdentifier,
        // colon: TokenSeparator,
        pub arg_type: LiteralTypes,
    }
}
pub mod literal_types {
    #[derive(PartialEq, Debug, Clone)]
    pub enum LiteralTypes {
        Int(IntType),
        Float(FloatType),
        Bool,
        // Bool(BoolType),
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum IntType {
        I8,
        I16,
        I32,
        I64,
        U8,
        U16,
        U32,
        U64,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum FloatType {
        F32,
        F64,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum BoolType {
        True,
        False,
    }
}
