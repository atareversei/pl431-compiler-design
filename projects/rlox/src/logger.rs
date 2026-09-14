use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LogSection {
    LexerTokens,
    ParserConsumedTokens,
    ParserStatements,
    ParserFuncInfo,
}

pub struct Logger {
    pub lexer_tokens: bool,
    pub parser_consumed_tokens: bool,
    pub parser_statements: bool,
    pub parser_func_info: bool,
}

impl Logger {
    pub fn enabled(&self, section: LogSection) -> bool {
        match section {
            LogSection::LexerTokens => self.lexer_tokens,
            LogSection::ParserConsumedTokens => self.parser_consumed_tokens,
            LogSection::ParserStatements => self.parser_statements,
            LogSection::ParserFuncInfo => self.parser_func_info,
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
