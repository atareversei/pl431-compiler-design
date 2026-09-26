use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::log;
use crate::logger::{LogSection, Logger};
use crate::lox::RunMode::File;
use crate::{error::LoxError, parser::Parser};
use std::{
    env, fs,
    io::{self, Write},
};

pub fn run() -> Result<(), Vec<LoxError>> {
    let config = parse_args();

    match config {
        Ok(cfg) => {
            let logger = Logger::new(&cfg);
            match cfg.mode {
                RunMode::Repl => run_repl(logger),
                RunMode::File(files) => run_file(files, logger),
            }
        }
        Err(err) => Err(vec![err]),
    }
}

fn run_repl(logger: Logger) -> Result<(), Vec<LoxError>> {
    let mut environment = Environment::globals();
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
            return Err(parse_result.errors);
        }

        let mut interpreter = Interpreter::new(parse_result.statements, environment, &logger);
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

fn run_file(paths: Vec<String>, logger: Logger) -> Result<(), Vec<LoxError>> {
    // TODO: add multiple files support
    let path = &paths[0];

    let bytes = fs::read(path).map_err(|err| vec![err.into()])?;
    let text = String::from_utf8_lossy(&bytes);

    let mut lexer = Lexer::new(&text);
    let lex_result = lexer.lex_tokens();
    if lex_result.has_errors() {}

    let mut parser = Parser::new(&lex_result.tokens, &logger);
    let parse_result = parser.parse();
    if parse_result.has_errors() {
        log!(logger, LogSection::GeneralInfo, "execution stopped");
        log!(
            logger,
            LogSection::ParserStatements,
            "Program Statements: {:?}",
            parse_result.statements
        );
        return Err(parse_result.errors);
    }

    let mut environment = Environment::globals();
    let mut interpreter = Interpreter::new(parse_result.statements, environment, &logger);
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

pub enum RunMode {
    Repl,
    File(Vec<String>),
}

pub struct Config {
    pub mode: RunMode,

    pub log_general_info: bool,
    pub log_parser_statements: bool,
    pub log_parser_consumed_tokens: bool,
    pub log_parser_func_info: bool,
    pub log_runtime_func_info_statements: bool,
    pub log_runtime_func_info_expressions: bool,
}

pub fn parse_args() -> Result<Config, LoxError> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut config = Config {
        mode: RunMode::Repl,
        log_general_info: true,
        log_parser_consumed_tokens: false,
        log_parser_statements: false,
        log_parser_func_info: false,
        log_runtime_func_info_statements: false,
        log_runtime_func_info_expressions: false,
    };

    for arg in args {
        match arg.as_str() {
            "--log-parser-consumed-tokens" => config.log_parser_consumed_tokens = true,
            "--log-parser-statements" => config.log_parser_statements = true,
            "--log-parser-func-info" => config.log_parser_func_info = true,
            "--log-runtime-func-info-statements" => config.log_runtime_func_info_statements = true,
            "--log-runtime-func-info-expressions" => {
                config.log_runtime_func_info_expressions = true
            }
            value if value.starts_with('-') => {
                return Err(LoxError::Config {
                    message: format!("undefined argument: {}", value),
                });
            }
            path => {
                let exists_result = fs::exists(path);

                match exists_result {
                    Ok(exists) => {
                        if !exists {
                            return Err(LoxError::Config {
                                message: format!("file does not exists: {}", path),
                            });
                        }
                        match &mut config.mode {
                            RunMode::Repl => {
                                config.mode = File(vec![path.to_string()]);
                            }
                            RunMode::File(files) => {
                                files.push(path.to_string());
                            }
                        }
                    }
                    Err(err) => {
                        return Err(LoxError::Config {
                            message: format!("filesystem error: {}", err),
                        });
                    }
                }
            }
        }
    }

    Ok(config)
}
