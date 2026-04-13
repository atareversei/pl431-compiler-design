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
        if self.peek() == expected {
            self.advance();
            return self.new_token(together);
        }

        self.new_token(first_to_go)
    }

    fn comment(&mut self) -> Result<Option<Token>, LoxError> {
        if self.peek() == '/' {
            self.advance();
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            Ok(None)
        } else if self.peek() == '*' {
            self.advance();
            let mut depth = 0;
            let mut c: char;
            while !self.is_at_end() && depth >= 0 {
                c = self.advance();
                match c {
                    '/' => {
                        if self.peek() == '*' {
                            self.advance();
                            depth += 1;
                        }
                    }
                    '*' => {
                        if self.peek() == '/' {
                            self.advance();
                            depth -= 1;
                        }
                    }
                    _ => {}
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let src = "\
        12+43.1=55.1
        2.1*6=12.6
        3/1=3
        0-1=-1
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();

        assert!(!lex_result.has_errors(), "lexing result must be errorless");

        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Number, 0, 2),
                Token::new(TT::Plus, 2, 1),
                Token::new(TT::Number, 3, 4),
                Token::new(TT::Equal, 7, 1),
                Token::new(TT::Number, 8, 4),
                Token::new(TT::Number, 21, 3),
                Token::new(TT::Star, 24, 1),
                Token::new(TT::Number, 25, 1),
                Token::new(TT::Equal, 26, 1),
                Token::new(TT::Number, 27, 4),
                Token::new(TT::Number, 40, 1),
                Token::new(TT::Slash, 41, 1),
                Token::new(TT::Number, 42, 1),
                Token::new(TT::Equal, 43, 1),
                Token::new(TT::Number, 44, 1),
                Token::new(TT::Number, 54, 1),
                Token::new(TT::Minus, 55, 1),
                Token::new(TT::Number, 56, 1),
                Token::new(TT::Equal, 57, 1),
                Token::new(TT::Minus, 58, 1),
                Token::new(TT::Number, 59, 1),
                Token::new(TT::Eof, 60, 0),
            ]
        );
    }

    #[test]
    fn comments() {
        let src = "\
        //                      single-line comment
        /// // /**/ a = 12      slightly complex single-line comment
        /*                      multi-line comment
        */
        /*
            /*
                /*
                                nested multi-line comment
                */
            */
        */
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();

        for t in &lex_result.tokens {
            println!("{}", t);
        }

        assert!(!lex_result.has_errors(), "lexing result must be errorless");

        assert_eq!(lex_result.tokens, vec![Token::new(TT::Eof, 322, 0),]);
    }
}
