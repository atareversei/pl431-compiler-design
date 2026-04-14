mod printer;
mod error;
mod expression;
mod lexer;
mod lox;
mod token;

use std::process;

fn main() {
    if let Err(err) = lox::run() {
        err.iter().for_each(|e| eprintln!("Error: {}", e));
        process::exit(1);
    }
}
