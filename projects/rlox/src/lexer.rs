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
    m.insert("class", TT::Class);
    m.insert("else", TT::Else);
    m.insert("false", TT::False);
    m.insert("for", TT::For);
    m.insert("func", TT::Func);
    m.insert("if", TT::If);
    m.insert("null", TT::Null);
    m.insert("return", TT::Return);
    m.insert("super", TT::Super);
    m.insert("this", TT::This);
    m.insert("true", TT::True);
    m.insert("var", TT::Var);
    m.insert("print", TT::Print); // TODO: remove print statement
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

type LexResultFn = Result<Option<Token>, LoxError>;

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

        tokens.push(Token::new(TT::Eof, self.current, 0, String::from("<Eof>")));
        LexResult { tokens, errors }
    }

    fn lex_token(&mut self) -> LexResultFn {
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
            ':' => Ok(Some(self.new_token(TT::Colon))),
            ';' => Ok(Some(self.new_token(TT::SemiColon))),
            '*' => Ok(Some(self.new_token(TT::Star))),
            '?' => Ok(Some(self.new_token(TT::Question))),
            '!' => Ok(Some(self.go_together('=', TT::BangEqual, TT::Bang))),
            '=' => Ok(Some(self.go_together('=', TT::EqualEqual, TT::Equal))),
            '<' => Ok(Some(self.go_together('=', TT::LessEqual, TT::Less))),
            '>' => Ok(Some(self.go_together('=', TT::GreaterEqual, TT::Greater))),
            '&' => Ok(Some(self.go_together('&', TT::AmpAmp, TT::Amp))),
            '|' => Ok(Some(self.go_together('|', TT::PipePipe, TT::Pipe))),
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

    fn comment(&mut self) -> LexResultFn {
        if self.peek() == '/' {
            self.single_line_comment()
        } else if self.peek() == '*' {
            self.multi_line_comment()
        } else {
            Ok(Some(self.new_token(TT::Slash)))
        }
    }

    fn single_line_comment(&mut self) -> LexResultFn {
        self.advance();
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
        Ok(None)
    }

    fn multi_line_comment(&mut self) -> LexResultFn {
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

    fn string(&mut self) -> LexResultFn {
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

        Ok(Some(self.new_token(TT::String)))
    }

    fn number(&mut self) -> LexResultFn {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        Ok(Some(self.new_token(TT::Number)))
    }

    fn identifier(&mut self) -> LexResultFn {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let val = &self.source[self.start..self.current];
        let tt = KEYWORDS.get(val);
        if let Some(tt) = tt {
            Ok(Some(self.new_token(*tt)))
        } else {
            Ok(Some(self.new_token(TT::Identifier)))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn token_len(&self) -> usize {
        self.current - self.start
    }

    fn new_token(&self, token_type: TT) -> Token {
        Token::new(
            token_type,
            self.start,
            self.token_len(),
            self.source[self.start..self.current].to_string(),
        )
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

    fn types(tokens: &[Token]) -> Vec<TT> {
        tokens.iter().map(|t| t.token_type).collect()
    }

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
            types(&lex_result.tokens),
            vec![
                TT::Number,
                TT::Plus,
                TT::Number,
                TT::Equal,
                TT::Number,
                TT::Number,
                TT::Star,
                TT::Number,
                TT::Equal,
                TT::Number,
                TT::Number,
                TT::Slash,
                TT::Number,
                TT::Equal,
                TT::Number,
                TT::Number,
                TT::Minus,
                TT::Number,
                TT::Equal,
                TT::Minus,
                TT::Number,
                TT::Eof,
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
        assert_eq!(types(&lex_result.tokens), vec![TT::Eof,]);
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
            types(&lex_result.tokens),
            vec![TT::Identifier, TT::Equal, TT::String, TT::Eof,]
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
            types(&lex_result.tokens),
            vec![TT::Identifier, TT::Equal, TT::Eof,]
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
            types(&lex_result.tokens),
            vec![
                TT::Number,
                TT::Greater,
                TT::Number,
                TT::Number,
                TT::GreaterEqual,
                TT::Number,
                TT::String,
                TT::EqualEqual,
                TT::String,
                TT::Number,
                TT::Less,
                TT::Number,
                TT::Number,
                TT::LessEqual,
                TT::Number,
                TT::Eof,
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
        if (sum(5,6) != 11 || false {
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
            types(&lex_result.tokens),
            vec![
                TT::Import,
                TT::String,
                TT::Var,
                TT::Identifier,
                TT::Func,
                TT::Identifier,
                TT::LParen,
                TT::Identifier,
                TT::Comma,
                TT::Identifier,
                TT::RParen,
                TT::LBrace,
                TT::Return,
                TT::Identifier,
                TT::Plus,
                TT::Identifier,
                TT::RBrace,
                TT::If,
                TT::LParen,
                TT::Identifier,
                TT::LParen,
                TT::Number,
                TT::Comma,
                TT::Number,
                TT::RParen,
                TT::BangEqual,
                TT::Number,
                TT::PipePipe,
                TT::False,
                TT::LBrace,
                TT::Identifier,
                TT::Equal,
                TT::String,
                TT::RBrace,
                TT::Class,
                TT::Identifier,
                TT::LBrace,
                TT::Func,
                TT::Identifier,
                TT::LParen,
                TT::RParen,
                TT::LBrace,
                TT::RBrace,
                TT::RBrace,
                TT::Identifier,
                TT::Dot,
                TT::Identifier,
                TT::LParen,
                TT::RParen,
                TT::Eof,
            ]
        );
    }

    #[test]
    fn unknown_tokens() {
        let src = "\
        name ^ 8
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
            types(&lex_result.tokens),
            vec![TT::Identifier, TT::Number, TT::Eof,]
        );
    }
}
