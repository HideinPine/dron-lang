pub mod types {
    #[derive(Debug, PartialEq, Clone)]
    pub enum LexerCartegories {
        Keyword(TokenKeyword), /* 3rd: PAUSE -- ALMOST DONE */
        Block(TokenBlock),
        Separator(TokenSeparator), /* 1st: PAUSE -- ALMOST DONE*/
        Literal(TokenLiteral),     /* LATER ON WHEN NEEDED */
        Operator(TokenOperator),   /* 2nd: PAUSE -- ALMOST DONE */
        Identifier(TokenIdentifier),
        Compare(TokenCompare),
        Assign(TokenAssign),
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
        Boolean(bool),
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
    #[derive(Debug)]
    pub struct Arguments {
        pub name: TokenIdentifier,
        // colon: TokenSeparator,
        pub arg_type: TokenIdentifier,
    }
}
