use std::fmt;
use std::io;

pub enum LoxError {
    Scan {
        message: String,
    },
    Lex {
        message: String,
        offset: isize,
        length: isize,
    },
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::Scan { message } => write!(f, "{}", message),
            LoxError::Lex {
                message,
                offset,
                length,
            } => write!(f, "{}", message),
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
