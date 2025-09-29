#[derive(Debug, Clone)]
pub struct Token {
    pub what: String,
}

pub struct Lexer<'a> {
    lexed: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(lexed: &'a str) -> Self {
        Self { lexed }
    }

    pub fn lex_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
