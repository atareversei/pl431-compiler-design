mod callable;
mod environment;
mod error;
mod expression;
mod interpreter;
mod lexer;
mod lox;
mod parser;
mod printer;
mod statement;
mod token;

use std::process;

fn main() {
    if let Err(err) = lox::run() {
        err.iter().for_each(|e| eprintln!("Error: {}", e));
        process::exit(1);
    }
}
