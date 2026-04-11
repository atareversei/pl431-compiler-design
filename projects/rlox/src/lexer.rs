use crate::error::LoxError;
use crate::token::{Token, TokenType as TT};
use std::collections::HashMap;
use std::sync::LazyLock;

static KEYWORDS: LazyLock<HashMap<&'static str, TT>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TT::And);
    m.insert("class", TT::Class);
    m.insert("else", TT::Else);
    m.insert("false", TT::False);
    m.insert("for", TT::For);
    m.insert("func", TT::Func);
    m.insert("if", TT::If);
    m.insert("null", TT::Null);
    m.insert("or", TT::Or);
    m.insert("return", TT::Return);
    m.insert("super", TT::Super);
    m.insert("this", TT::This);
    m.insert("true", TT::True);
    m.insert("var", TT::Var);
    m
});

pub struct LexResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<LoxError>,
}

impl LexResult {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub struct Lexer<'a> {
    source: &'a str,

    start: usize,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
        }
    }

    pub fn lex_tokens(&mut self) -> LexResult {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            match self.lex_token() {
                Ok(res) => {
                    if let Some(token) = res {
                        tokens.push(token);
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        tokens.push(Token::new(TT::Eof, self.current, 0));
        LexResult { tokens, errors }
    }

    fn lex_token(&mut self) -> Result<Option<Token>, LoxError> {
        let c = self.advance();

        let s = self.start;
        let l = self.token_len();

        match c {
            '(' => Ok(Some(self.new_token(TT::LParen))),
            ')' => Ok(Some(self.new_token(TT::RParen))),
            '{' => Ok(Some(self.new_token(TT::LBrace))),
            '}' => Ok(Some(self.new_token(TT::RBrace))),
            ',' => Ok(Some(self.new_token(TT::Comma))),
            '.' => Ok(Some(self.new_token(TT::Dot))),
            '-' => Ok(Some(self.new_token(TT::Minus))),
            '+' => Ok(Some(self.new_token(TT::Plus))),
            ';' => Ok(Some(self.new_token(TT::SemiColon))),
            '*' => Ok(Some(self.new_token(TT::Star))),
            '!' => Ok(Some(self.go_together('=', TT::BangEqual, TT::Bang))),
            '=' => Ok(Some(self.go_together('=', TT::EqualEqual, TT::Equal))),
            '<' => Ok(Some(self.go_together('=', TT::LessEqual, TT::Less))),
            '>' => Ok(Some(self.go_together('=', TT::GreaterEqual, TT::Greater))),
            '/' => self.comment(),
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\t' | '\r' | '\n' => Ok(None),
            _ => Err(LoxError::Lex {
                message: String::from("unknown token"),
                offset: s,
                length: l,
            }),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn expect_cur(&mut self, expected: char) -> bool {
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn go_together(&mut self, expected: char, together: TT, first_to_go: TT) -> Token {
        if self.is_at_end() {
            return self.new_token(first_to_go);
        }
        if self.expect_cur(expected) {
            return self.new_token(first_to_go);
        }

        self.new_token(together)
    }

    fn comment(&mut self) -> Result<Option<Token>, LoxError> {
        if self.expect_cur('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            Ok(None)
        } else {
            Ok(Some(self.new_token(TT::Slash)))
        }
    }

    fn string(&mut self) -> Result<Option<Token>, LoxError> {
        while self.peek() != '"' || !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::Lex {
                message: String::from("unterminated string"),
                offset: self.start,
                length: self.token_len(),
            });
        }

        // consume `"`.
        self.advance();

        Ok(Some(Token::new(TT::String, self.start, self.token_len())))
    }

    fn number(&mut self) -> Result<Option<Token>, LoxError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        Ok(Some(Token::new(TT::Number, self.start, self.token_len())))
    }

    fn identifier(&mut self) -> Result<Option<Token>, LoxError> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let val = &self.source[self.start..self.current];
        let tt = KEYWORDS.get(val);
        if let Some(tt) = tt {
            Ok(Some(self.new_token(*tt)))
        } else {
            Ok(Some(Token::new(
                TT::Identifier,
                self.start,
                self.token_len(),
            )))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn token_len(&self) -> usize {
        self.current - self.start
    }

    fn new_token(&self, token_type: TT) -> Token {
        Token::new(token_type, self.start, self.token_len())
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }
}
