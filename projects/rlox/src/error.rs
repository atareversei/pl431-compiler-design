use std::fmt;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoxError {
    Scan {
        message: String,
    },
    Lex {
        message: String,
        offset: usize,
        length: usize,
    },
    // TODO: add token at which the error has occurred
    Parse {
        message: String,
    },
    Runtime {
        message: String,
    },
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Scan { message } => write!(f, "ScanError: {}", message),
            LoxError::Lex {
                message,
                offset,
                length,
            } => write!(f, "LexError: {} {}:{}", message, offset, length),
            LoxError::Parse { message } => write!(f, "ParseError: {}", message),
            LoxError::Runtime { message } => write!(f, "RuntimeError: {}", message),
        }
    }
}

impl From<io::Error> for LoxError {
    fn from(err: io::Error) -> Self {
        LoxError::Scan {
            message: format!("{}", err),
        }
    }
}
