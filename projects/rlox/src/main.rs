mod error;
mod lexer;
mod lox;
mod token;

use std::process;

fn main() {
    if let Err(err) = lox::run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
