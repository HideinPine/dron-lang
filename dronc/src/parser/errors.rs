pub mod name_error {
    use crate::lexer::types::{LexerCartegories, TokenIdentifier};
    use owo_colors::OwoColorize;
    pub fn name_error(value: Option<&LexerCartegories>, expected: TokenIdentifier, pos: usize) {
        println!(
            "{}{:?} expected {:?}{}{}",
            "Error Found: ".bold().red(),
            value.unwrap().bold().red(),
            expected.bold().red(),
            " at position: ".red().bold(),
            pos.bold()
        );
        exit();
    }
    #[allow(unused)]
    fn exit() {}
}
