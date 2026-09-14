use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::logger::Logger;
use crate::{error::LoxError, parser::Parser};
use std::{
    env, fs,
    io::{self, Write},
};

// TODO: add debug features that are enabled with flags

pub fn run() -> Result<(), Vec<LoxError>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_repl()
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        Result::Err(vec![LoxError::Scan {
            message: "Usage: rlox [script]".to_string(),
        }])
    }
}

fn run_repl() -> Result<(), Vec<LoxError>> {
    let logger = Logger {
        lexer_tokens: false,
        parser_consumed_tokens: true,
        parser_func_info: false,
        parser_statements: false,
    };

    let mut environment = Environment::new();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().map_err(|err| vec![err.into()])?;
        line.clear();
        let bytes = stdin.read_line(&mut line).map_err(|err| vec![err.into()])?;
        if bytes == 0 {
            break;
        }

        let mut lexer = Lexer::new(&line);
        let lex_result = lexer.lex_tokens();
        if lex_result.has_errors() {}

        let mut parser = Parser::new(&lex_result.tokens, &logger);
        let parse_result = parser.parse();
        if parse_result.has_errors() {
            println!("execution stopped");
            println!("Program Statements: {:?}", parse_result.statements);
            return Err(parse_result.errors);
        }

        let mut interpreter = Interpreter::new(parse_result.statements, environment);
        match interpreter.interpret() {
            Ok(ctx) => {
                environment = ctx.environment.borrow().clone();

                if let Some(v) = ctx.last_expr_value {
                    println!("{:?}", v);
                }
            }
            Err(err) => {
                println!("{err}");
                return Err(vec![err]);
            }
        };
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), Vec<LoxError>> {
    // TODO: add environmental variables
    let logger = Logger {
        lexer_tokens: false,
        parser_consumed_tokens: true,
        parser_func_info: false,
        parser_statements: false,
    };

    let bytes = fs::read(path).map_err(|err| vec![err.into()])?;
    let text = String::from_utf8_lossy(&bytes);

    let mut lexer = Lexer::new(&text);
    let lex_result = lexer.lex_tokens();
    if lex_result.has_errors() {}

    let mut parser = Parser::new(&lex_result.tokens, &logger);
    let parse_result = parser.parse();
    if parse_result.has_errors() {
        println!("execution stopped");
        println!("Program Statements: {:?}", parse_result.statements);
        return Err(parse_result.errors);
    }

    let environment = Environment::new();
    let mut interpreter = Interpreter::new(parse_result.statements, environment);
    match interpreter.interpret() {
        Ok(ctx) => {
            println!("program executed successfully");
        }
        Err(err) => {
            println!("{err}");
            return Err(vec![err]);
        }
    };
    Ok(())
}
