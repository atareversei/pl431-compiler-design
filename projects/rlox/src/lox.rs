use crate::error::LoxError;
use crate::lexer::Lexer;
use std::{
    env, fs,
    io::{self, Write},
};

pub fn run() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl()
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        Result::Err(LoxError::Scan {
            message: "Usage: rlox [script]".to_string(),
        })
    }
}

fn run_repl() -> Result<(), LoxError> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        line.clear();
        let bytes = stdin.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
    }

    start(&line)
}

fn run_file(path: &str) -> Result<(), LoxError> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes);
    start(&text)
}

fn start(source: &str) -> Result<(), LoxError> {
    let mut lexer = Lexer::new(source);
    let lex_result = lexer.lex_tokens();

    for token in lex_result.tokens {
        println!("{}", token);
    }

    Ok(())
}
