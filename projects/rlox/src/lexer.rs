use crate::error::LoxError;
use crate::token::{Token, TokenType as TT};
use std::collections::HashMap;
use std::sync::LazyLock;

// TODO:
// 1. binary literals      e.g. 0b110101
// 2. hex literals         e.g. 0xFF
// 3. unicode identifiers  e.g. café
// 4. string interpolation e.g. "The total is: ${a + b}"
// 5. escape sequences     e.g. "tab:\t quote: \" unicode: \u03A9"

static KEYWORDS: LazyLock<HashMap<&'static str, TT>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("import", TT::Import);
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
            self.single_line_comment()
        } else if self.peek() == '*' {
            self.multi_line_comment()
        } else {
            Ok(Some(self.new_token(TT::Slash)))
        }
    }

    fn single_line_comment(&mut self) -> Result<Option<Token>, LoxError> {
        self.advance();
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
        Ok(None)
    }

    fn multi_line_comment(&mut self) -> Result<Option<Token>, LoxError> {
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
    }

    fn string(&mut self) -> Result<Option<Token>, LoxError> {
        while !self.is_at_end() && self.peek() != '"' {
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
        assert!(!lex_result.has_errors(), "lexing result must be errorless");
        assert_eq!(lex_result.tokens, vec![Token::new(TT::Eof, 322, 0),]);
    }

    #[test]
    fn strings() {
        let src = "\
        name = \"lox\"
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();
        assert!(!lex_result.has_errors(), "lexing result must be errorless");
        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Identifier, 0, 4),
                Token::new(TT::Equal, 5, 1),
                Token::new(TT::String, 7, 5),
                Token::new(TT::Eof, 12, 0),
            ]
        );
    }

    #[test]
    fn unterminated_strings() {
        let src = "\
        name = \"lox
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();
        assert!(lex_result.has_errors(), "lexing result must have an error");
        assert_eq!(
            lex_result.errors,
            vec![LoxError::Lex {
                message: String::from("unterminated string"),
                offset: 7,
                length: 4
            },]
        );
        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Identifier, 0, 4),
                Token::new(TT::Equal, 5, 1),
                Token::new(TT::Eof, 11, 0),
            ]
        );
    }

    #[test]
    fn comparisons() {
        let src = "\
        12>99
        1>=1
        \"\"==\"\"
        0.0<7
        3<=4
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();

        assert!(!lex_result.has_errors(), "lexing result must be errorless");
        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Number, 0, 2),
                Token::new(TT::Greater, 2, 1),
                Token::new(TT::Number, 3, 2),
                Token::new(TT::Number, 14, 1),
                Token::new(TT::GreaterEqual, 15, 2),
                Token::new(TT::Number, 17, 1),
                Token::new(TT::String, 27, 2),
                Token::new(TT::EqualEqual, 29, 2),
                Token::new(TT::String, 31, 2),
                Token::new(TT::Number, 42, 3),
                Token::new(TT::Less, 45, 1),
                Token::new(TT::Number, 46, 1),
                Token::new(TT::Number, 56, 1),
                Token::new(TT::LessEqual, 57, 2),
                Token::new(TT::Number, 59, 1),
                Token::new(TT::Eof, 60, 0),
            ]
        );
    }

    #[test]
    fn simple_program() {
        let src = "\
        import \"./a.lox\"
        
        /*
            status shows the overall condition of the program
        */
        var status
        func sum(a, b) {
            return a + b
        }

        // set status if sum function is not working correctly
        if (sum(5,6) != 11 or false {
            status = \"panic\"
        }

        class system {
            func exit(){/*TODO*/}
        }
        system.exit()
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();

        assert!(!lex_result.has_errors(), "lexing result must be errorless");
        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Import, 0, 6),
                Token::new(TT::String, 7, 9),
                Token::new(TT::Var, 118, 3),
                Token::new(TT::Identifier, 122, 6),
                Token::new(TT::Func, 137, 4),
                Token::new(TT::Identifier, 142, 3),
                Token::new(TT::LParen, 145, 1),
                Token::new(TT::Identifier, 146, 1),
                Token::new(TT::Comma, 147, 1),
                Token::new(TT::Identifier, 149, 1),
                Token::new(TT::RParen, 150, 1),
                Token::new(TT::LBrace, 152, 1),
                Token::new(TT::Return, 166, 6),
                Token::new(TT::Identifier, 173, 1),
                Token::new(TT::Plus, 175, 1),
                Token::new(TT::Identifier, 177, 1),
                Token::new(TT::RBrace, 187, 1),
                Token::new(TT::If, 261, 2),
                Token::new(TT::LParen, 264, 1),
                Token::new(TT::Identifier, 265, 3),
                Token::new(TT::LParen, 268, 1),
                Token::new(TT::Number, 269, 1),
                Token::new(TT::Comma, 270, 1),
                Token::new(TT::Number, 271, 1),
                Token::new(TT::RParen, 272, 1),
                Token::new(TT::BangEqual, 274, 2),
                Token::new(TT::Number, 277, 2),
                Token::new(TT::Or, 280, 2),
                Token::new(TT::False, 283, 5),
                Token::new(TT::LBrace, 289, 1),
                Token::new(TT::Identifier, 303, 6),
                Token::new(TT::Equal, 310, 1),
                Token::new(TT::String, 312, 7),
                Token::new(TT::RBrace, 328, 1),
                Token::new(TT::Class, 339, 5),
                Token::new(TT::Identifier, 345, 6),
                Token::new(TT::LBrace, 352, 1),
                Token::new(TT::Func, 366, 4),
                Token::new(TT::Identifier, 371, 4),
                Token::new(TT::LParen, 375, 1),
                Token::new(TT::RParen, 376, 1),
                Token::new(TT::LBrace, 377, 1),
                Token::new(TT::RBrace, 386, 1),
                Token::new(TT::RBrace, 396, 1),
                Token::new(TT::Identifier, 406, 6),
                Token::new(TT::Dot, 412, 1),
                Token::new(TT::Identifier, 413, 4),
                Token::new(TT::LParen, 417, 1),
                Token::new(TT::RParen, 418, 1),
                Token::new(TT::Eof, 419, 0),
            ]
        );
    }

    #[test]
    fn unknown_tokens() {
        let src = "\
        name ? 8
        "
        .trim();

        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();
        assert!(lex_result.has_errors(), "lexing result must have an error");
        assert_eq!(
            lex_result.errors,
            vec![LoxError::Lex {
                message: String::from("unknown token"),
                offset: 5,
                length: 1
            },]
        );
        assert_eq!(
            lex_result.tokens,
            vec![
                Token::new(TT::Identifier, 0, 4),
                Token::new(TT::Number, 7, 1),
                Token::new(TT::Eof, 8, 0),
            ]
        );
    }
}
