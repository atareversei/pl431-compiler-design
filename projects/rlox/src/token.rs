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
    Question,

    // Single or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Amp,
    AmpAmp,
    Pipe,
    PipePipe,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    Import,
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
    Print, // TODO: remove print statements

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

impl TokenType {
    /// Returns the lexeme for this token type (test-only utility)
    #[cfg(test)]
    pub fn test_lexeme(&self) -> &'static str {
        match self {
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Star => "*",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Slash => "/",
            TokenType::Dot => ".",
            TokenType::Comma => ",",
            TokenType::SemiColon => ";",
            TokenType::Colon => ":",
            TokenType::Question => "?",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Amp => "&",
            TokenType::AmpAmp => "&&",
            TokenType::Pipe => "|",
            TokenType::PipePipe => "||",
            TokenType::Identifier => "identifier",
            TokenType::String => "string",
            TokenType::Number => "number",
            TokenType::Import => "import",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Var => "var",
            TokenType::Null => "null",
            TokenType::For => "for",
            TokenType::Func => "func",
            TokenType::Return => "return",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Class => "class",
            TokenType::Super => "super",
            TokenType::This => "this",
            TokenType::Print => "print",
            TokenType::Eof => "EOF",
        }
    }
}
