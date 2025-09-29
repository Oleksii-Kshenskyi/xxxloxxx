use anyhow::Result;

use std::{
    io::{BufRead, Write},
    path::Path,
};

use crate::lexer::Lexer;

pub struct Lox;

impl Lox {
    pub fn run_file(path: &str) -> Result<()> {
        let contents = std::fs::read_to_string(Path::new(path))?;
        Lox::run(&contents)
    }

    pub fn run_prompt() -> Result<()> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        for line in stdin.lock().lines() {
            write!(stdout, "<XXXLOXXX>> ")?;
            stdout.flush()?;

            let l = line?;
            Self::run(&l.trim())?;
        }

        Ok(())
    }

    fn run(what: &str) -> Result<()> {
        let lexer = Lexer::new(what);
        let tokens = lexer.lex_tokens();

        println!(
            "[{}]",
            tokens
                .iter()
                .map(|t| t.what.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );

        Ok(())
    }
}
