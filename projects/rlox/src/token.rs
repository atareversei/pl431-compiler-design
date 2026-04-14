use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    offset: usize,
    length: usize,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, offset: usize, length: usize, lexeme: String) -> Self {
        Token {
            token_type,
            offset,
            length,
            lexeme,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.token_type, self.offset, self.length)
    }
}

/// TokenType defines all the tokens of RLox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Identifier,
    String,
    Number,

    // Keywords.
    Import,
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
            _ => write!(f, "{:?}", self),
        }
    }
}
