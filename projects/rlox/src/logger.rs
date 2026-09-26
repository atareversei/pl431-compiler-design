use core::fmt;
use std::env;

use crate::{logger, lox::Config};

#[derive(Debug, Clone, Copy)]
pub enum LogSection {
    GeneralInfo,
    LexerTokens,
    ParserConsumedTokens,
    ParserStatements,
    ParserFuncInfo,
    RuntimeFuncInfoStatements,
    RuntimeFuncInfoExpressions,
}

pub struct Logger {
    pub general_info: bool,
    pub lexer_tokens: bool,
    pub parser_consumed_tokens: bool,
    pub parser_statements: bool,
    pub parser_func_info: bool,
    pub runtime_func_info_statements: bool,
    pub runtime_func_info_expressions: bool,
}

impl Logger {
    pub fn new(config: &Config) -> Self {
        Self {
            general_info: true,
            lexer_tokens: false,
            parser_consumed_tokens: config.log_parser_consumed_tokens,
            parser_statements: config.log_parser_statements,
            parser_func_info: config.log_parser_func_info,
            runtime_func_info_statements: config.log_runtime_func_info_statements,
            runtime_func_info_expressions: config.log_runtime_func_info_expressions,
        }
    }

    pub fn new_all_off() -> Self {
        Self {
            general_info: false,
            lexer_tokens: false,
            parser_consumed_tokens: false,
            parser_statements: false,
            parser_func_info: false,
            runtime_func_info_statements: false,
            runtime_func_info_expressions: false,
        }
    }

    pub fn enabled(&self, section: LogSection) -> bool {
        match section {
            LogSection::GeneralInfo => self.general_info,
            LogSection::LexerTokens => self.lexer_tokens,
            LogSection::ParserConsumedTokens => self.parser_consumed_tokens,
            LogSection::ParserStatements => self.parser_statements,
            LogSection::ParserFuncInfo => self.parser_func_info,
            LogSection::RuntimeFuncInfoStatements => self.runtime_func_info_statements,
            LogSection::RuntimeFuncInfoExpressions => self.runtime_func_info_expressions,
        }
    }

    pub fn log(&self, section: LogSection, args: fmt::Arguments<'_>) {
        println!("[{section:?}] {args}");
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $section:expr, $($arg:tt)*) => {
        if $logger.enabled($section) {
            $logger.log(
                $section,
                format_args!($($arg)*)
            );
        }
    };
}

// #[macro_export] puts `log!` at the crate root.
// This also makes it available through `logger`.
pub use crate::log;
