pub mod tokens;

pub mod eval {
    use crate::lexer::tokens::types::{
        LexerCartegories::{self, Assign, Compare},
        TokenAssign, TokenBlock, TokenCompare, TokenKeyword, TokenOperator, TokenSeparator,
        TokenState,
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
    pub fn lex_assign_compare(char: char, next: Option<&char>) -> Option<LexerCartegories> {
        match (char, next) {
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
            ':' => Some(TokenSeparator::Colon),
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
            "ghost_block" => Some(TokenBlock::Ghost),
            "unsafe_block" => Some(TokenBlock::Unsafe),
            "eph_region" => Some(TokenBlock::EphemeralRegion),
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
    pub fn comp_state(state: &str) -> Option<TokenState> {
        match state {
            "let" => Some(TokenState::VariableDeclaration),
            _ => None,
        }
    }
    pub fn is_state(state: &str) -> bool {
        comp_state(state).is_some()
    }
}

pub mod tokenizer {
    use crate::lexer::{
        eval::{comp_operator, comp_sep, is_token_comp, is_token_oper, is_token_sep},
        flush::{flush_int, flush_keyword, flush_oper},
        tokens::types::LexerCartegories::{self, EndOfFile},
    };
    use crate::parser::errors::lexer_error::LexerError;
    pub fn tokenize(chars: Vec<char>) -> Result<Vec<LexerCartegories>, LexerError> {
        let mut keyword_value: Vec<char> = Vec::new();
        let mut oper_val: Vec<char> = Vec::new();
        let mut tokens: Vec<LexerCartegories> = Vec::new();
        let mut integer: Vec<char> = Vec::new();

        for (i, char) in chars.iter().enumerate() {
            if char.is_ascii_digit() {
                if !keyword_value.is_empty() {
                    keyword_value.push(*char);
                } else {
                    integer.push(*char);
                }
            } else if char.is_alphabetic() || !keyword_value.is_empty() && *char == '_' {
                keyword_value.push(*char);
                flush_oper(&mut oper_val, &mut tokens);
            } else if !integer.is_empty() && *char == '.' {
                integer.push(*char);
            } else if char.is_whitespace() {
                flush_int(&mut integer, &mut tokens)?;
                flush_keyword(&mut keyword_value, &mut tokens);
                flush_oper(&mut oper_val, &mut tokens);
            } else if is_token_sep(char) {
                flush_keyword(&mut keyword_value, &mut tokens);
                flush_int(&mut integer, &mut tokens)?;
                let sep = match comp_sep(char) {
                    Some(separator) => LexerCartegories::Separator(separator),
                    None => todo!(),
                };
                tokens.push(sep);
            } else if is_token_oper(char) {
                flush_int(&mut integer, &mut tokens)?;
                flush_keyword(&mut keyword_value, &mut tokens);
                let sub = *char;
                if *char == '-' && chars.get(i + 1).is_some_and(|char| *char == '>') {
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
                flush_int(&mut integer, &mut tokens)?;
                oper_val.push(*char);
                if oper_val.len() == 2 {
                    flush_oper(&mut oper_val, &mut tokens);
                }
            } else {
                todo!()
            }
        }
        flush_keyword(&mut keyword_value, &mut tokens);
        tokens.push(EndOfFile);
        Ok(tokens)
    }
}
pub mod flush {
    use crate::{
        errors::lexer_error::LexerError,
        lexer::{
            eval::{
                comp_block, comp_key, comp_state, is_block, is_keyword, is_state,
                lex_assign_compare,
            },
            tokens::types::{LexerCartegories, TokenIdentifier, TokenLiteral},
        },
    };
    pub fn flush_keyword(keyword: &mut Vec<char>, tokens: &mut Vec<LexerCartegories>) {
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
        } else if is_state(val.as_str()) {
            let statement = match comp_state(val.as_str()) {
                Some(state) => LexerCartegories::Statement(state),
                None => LexerCartegories::Identifier(TokenIdentifier::new(val.clone())),
            };
            tokens.push(statement);
            return;
        }
        tokens.push(LexerCartegories::Identifier(TokenIdentifier::new(val)));
    }
    pub fn flush_int(
        int: &mut Vec<char>,
        tokens: &mut Vec<LexerCartegories>,
    ) -> Result<(), LexerError> {
        if int.is_empty() {
            return Ok(());
        }
        let value: String = int.drain(..).collect();
        match value.matches('.').count() {
            0_usize => {
                let i = value.parse::<i64>().map_err(|_| LexerError::FloatError)?;
                tokens.push(LexerCartegories::Literal(TokenLiteral::Integer(i)))
            }
            1_usize => {
                let val = value.parse::<f64>().map_err(|_| LexerError::FloatError)?;
                tokens.push(LexerCartegories::Literal(TokenLiteral::Float(val)))
            }
            2_usize.. => return Err(LexerError::InvalidDigitError(value.clone())),
        }
        Ok(())
    }
    pub fn flush_oper(oper: &mut Vec<char>, tokens: &mut Vec<LexerCartegories>) {
        if oper.is_empty() {
            return;
        } else if !oper.is_empty() && oper.len() <= 2 {
            let (char, next) = (oper[0], oper.get(1));
            match lex_assign_compare(char, next) {
                Some(value) => tokens.push(value),
                None => println!("Unexpected symbol: {}", char),
            }
        } else {
            println!("Added char:{:?}", oper);
        }
        oper.clear()
    }
}
