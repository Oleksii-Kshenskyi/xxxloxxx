use std::process::ExitCode;

use anyhow::Result;

pub mod lexer;
pub mod lox;
use crate::lox::*;

fn main() -> Result<ExitCode> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Usage: `xxxloxxx <script file>`");
        return Ok(ExitCode::from(64));
    } else if args.len() == 1 {
        Lox::run_file(args.get(0).unwrap())?;
    } else {
        Lox::run_prompt()?;
    }

    Ok(ExitCode::SUCCESS)
}
