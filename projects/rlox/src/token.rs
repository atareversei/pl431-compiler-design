use std::fmt;

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    offset: usize,
    length: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, offset: usize, length: usize) -> Self {
        Token {
            token_type,
            lexeme,
            offset,
            length,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}:{}",
            self.token_type, self.lexeme, self.offset, self.length
        )
    }
}

/// TokenType defines all the tokens of RLox.
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Star,
    Plus,
    Minus,
    Slash,
    Dot,
    Comma,
    SemiColon,
    Colon,

    // Single or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Or,
    If,
    Else,
    Var,
    Null,
    For,
    Func,
    Return,
    True,
    False,
    Class,
    Super,
    This,

    // Other.
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Identifier(value) => write!(f, "Identifier({})", value),
            TokenType::String(value) => write!(f, "String({})", value),
            TokenType::Number(value) => write!(f, "Number({})", value),
            _ => write!(f, "{:?}", self),
        }
    }
}
