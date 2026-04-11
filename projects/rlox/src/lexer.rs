use crate::error::LoxError;
use crate::token::{Token, TokenType};

pub struct LexResult<'a> {
    pub tokens: Vec<Token<'a>>,
    pub errors: Vec<LoxError>,
}

pub struct Lexer<'a> {
    source: &'a str,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex_tokens(&mut self) -> LexResult {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            match self.lex_token() {
                Ok() =>
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", self.current, 0));
        LexResult { tokens, errors }
    }

    fn lex_token(&mut self) -> Result<Token, LoxError> {
        let c = self.advance();

        return Ok(Token::new(TokenType::And, "", 0, 0))
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.index(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
