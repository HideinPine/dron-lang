//#![allow(unused)]
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
    }
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenSeparator {
        LeftBrace,
        RightBrace,
        SemiColon,
        LeftCurl,
        RightCurl,
        Comma,
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
        Boolean,
        Integer,
        Float,
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
}

pub mod eval {
    use crate::lexer::types::{
        LexerCartegories::{self, Assign, Compare},
        TokenAssign, TokenBlock, TokenCompare, TokenKeyword, TokenOperator, TokenSeparator,
    };
    pub fn comp_assign(comp: char) -> Option<TokenAssign> {
        match comp {
            '=' => Some(TokenAssign::Equal),
            _ => None,
        }
    }
    pub fn is_token_assign(value: char) -> bool {
        let check = comp_assign(value);
        check.is_some()
    }
    pub fn lex_assign_compare(c: char, next: Option<&char>) -> Option<LexerCartegories> {
        match (c, next) {
            ('=', Some('=')) => Some(Compare(TokenCompare::DoubleEqual)),
            ('=', Some('>')) => Some(Assign(TokenAssign::DoubleArrow)),
            ('-', Some('>')) => Some(Assign(TokenAssign::SingleArrow)),
            ('<', Some('=')) => Some(Compare(TokenCompare::LessEqual)),
            ('>', Some('=')) => Some(Compare(TokenCompare::GreaterEqual)),
            ('!', Some('=')) | ('=', Some('!')) => Some(Compare(TokenCompare::NotEqual)),
            ('>', _) => Some(Compare(TokenCompare::Greater)),
            ('<', _) => Some(Compare(TokenCompare::Less)),
            ('=', _) => Some(Assign(TokenAssign::Equal)),
            _ => None,
        }
    }
    pub fn is_token_comp(comp: &char) -> bool {
        matches!(comp, '=' | '<' | '>' | '!')
    }
    pub fn comp_sep(pun: &char) -> Option<TokenSeparator> {
        match pun {
            '(' => Some(TokenSeparator::LeftBrace),
            ')' => Some(TokenSeparator::RightBrace),
            '{' => Some(TokenSeparator::LeftCurl),
            '}' => Some(TokenSeparator::RightCurl),
            ';' => Some(TokenSeparator::SemiColon),
            ',' => Some(TokenSeparator::Comma),
            _ => None,
        }
    }
    pub fn is_token_sep(pun: &char) -> bool {
        comp_sep(pun).is_some()
    }
    pub fn comp_operator(comp: &char) -> Option<TokenOperator> {
        match comp {
            '+' => Some(TokenOperator::Add),
            '-' => Some(TokenOperator::Sub),
            '/' => Some(TokenOperator::Div),
            '*' => Some(TokenOperator::Mul),
            _ => None,
        }
    }
    pub fn is_token_oper(oper: &char) -> bool {
        comp_operator(oper).is_some()
    }
    pub fn comp_block(block: &str) -> Option<TokenBlock> {
        match block {
            "ghost" => Some(TokenBlock::Ghost),
            "unsafe" => Some(TokenBlock::Unsafe),
            _ => None,
        }
    }
    pub fn is_block(block: &str) -> bool {
        comp_block(block).is_some()
    }
    pub fn comp_key(key: &str) -> Option<TokenKeyword> {
        match key {
            "fn" => Some(TokenKeyword::Function),
            "enum" => Some(TokenKeyword::Enum),
            "struct" => Some(TokenKeyword::Struct),
            _ => None,
        }
    }
    pub fn is_keyword(val: &str) -> bool {
        comp_key(val).is_some()
    }
}

pub mod tokenizer {
    use crate::lexer::{
        eval::{
            comp_block, comp_key, comp_operator, comp_sep, is_block, is_keyword, is_token_comp,
            is_token_oper, is_token_sep, lex_assign_compare,
        },
        types::{
            LexerCartegories::{self, EndOfFile},
            TokenIdentifier,
        },
    };
    use crate::parser::stmt::{Parser, try_val};
    use owo_colors::OwoColorize;
    pub fn tokenize(chars: Vec<char>) {
        let mut keyword_value: Vec<char> = Vec::new();
        let mut oper_val: Vec<char> = Vec::new();
        let mut tokens: Vec<LexerCartegories> = Vec::new();
        let mut newline_counter = 1;

        for (i, char) in chars.iter().enumerate() {
            if char.is_alphabetic() || !keyword_value.is_empty() && char.is_numeric() {
                keyword_value.push(*char);
                flush_oper(&mut oper_val, &mut tokens);
            } else if char.is_whitespace() {
                flush_keyword(&mut keyword_value, &mut tokens);
                flush_oper(&mut oper_val, &mut tokens);
                if *char == '\n' {
                    newline_counter += 1;
                }
            } else if is_token_sep(char) {
                flush_keyword(&mut keyword_value, &mut tokens);
                let sep = match comp_sep(char) {
                    Some(separator) => LexerCartegories::Separator(separator),
                    None => todo!(),
                };
                tokens.push(sep);
            } else if is_token_oper(char) {
                flush_keyword(&mut keyword_value, &mut tokens);
                let sub = *char;
                if *char == '-' && chars.get(i + 1).is_some_and(|c| *c == '>') {
                    oper_val.push(*char);
                    //oper_val.push(*chars.get(i + 1).unwrap());
                    //i += 1;
                    continue;
                } else if !oper_val.is_empty() {
                    flush_oper(&mut oper_val, &mut tokens);
                }
                match comp_operator(&sub) {
                    Some(operator) => {
                        let oper = LexerCartegories::Operator(operator);
                        tokens.push(oper)
                    }
                    None => todo!(),
                };
            } else if is_token_comp(char) {
                flush_keyword(&mut keyword_value, &mut tokens);
                oper_val.push(*char);
                if oper_val.len() == 2 {
                    flush_oper(&mut oper_val, &mut tokens);
                }
            } else {
                println!("Found smth else: {:?} on line {newline_counter}", char);
            }
        }
        flush_keyword(&mut keyword_value, &mut tokens);
        tokens.push(EndOfFile);
        let mut parser = Parser {
            tokens: &tokens,
            pos: 0,
        };
        match parser.parse_program() {
            Ok(smth) => println!("\n{:?}", smth.green().bold()),
            Err(er) => println!("{:?}", er),
        }
        println!("\n{:?}", tokens.blue().bold());
        println!("Total lines are: {newline_counter}\n");
        //keyword_type(TokenKeyword::Function, &tokens, 0);
        //println!("Function values: {:?}", function_values(&tokens, 8));
        try_val(&tokens);
        //println!("Operator value..: {:?} Should be empty", oper_val);
        /* println!("\n{:?}",tokens.blue().bold()); */
    }
    fn flush_keyword(keyword: &mut Vec<char>, tokens: &mut Vec<LexerCartegories>) {
        if keyword.is_empty() {
            return;
        }
        let val = keyword.drain(..).collect::<String>();
        if is_block(val.as_str()) {
            let value = match comp_block(val.as_str()) {
                Some(block) => LexerCartegories::Block(block),
                None => LexerCartegories::Identifier(TokenIdentifier::new(val)),
            };
            tokens.push(value);
            return;
        } else if is_keyword(val.as_str()) {
            let value = match comp_key(val.as_str()) {
                Some(keyword) => LexerCartegories::Keyword(keyword),
                None => LexerCartegories::Identifier(TokenIdentifier::new(val)),
            };
            tokens.push(value);
            return;
        }
        tokens.push(LexerCartegories::Identifier(TokenIdentifier::new(val)));
    }
    fn flush_oper(oper: &mut Vec<char>, tokens: &mut Vec<LexerCartegories>) {
        if oper.is_empty() {
            return;
        } else if !oper.is_empty() && oper.len() <= 2 {
            let (c, next) = (oper[0], oper.get(1));
            match lex_assign_compare(c, next) {
                Some(value) => tokens.push(value),
                None => println!("Unexpected symbol: {}", c),
            }
        } else {
            println!("Added char:{:?}", oper);
        }
        oper.clear()
    }
}
