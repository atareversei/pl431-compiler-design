use crate::{
    error::LoxError,
    expression::{Expression, LiteralValue},
    token::{Token, TokenType as TT},
};

// TODO: remove `clone()`

// Redesign output strategy
pub type ParseResult = Result<Expression, LoxError>;
type ParseResultFn = Result<Expression, LoxError>;

pub struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TT::SemiColon {
                return;
            }

            match self.peek().token_type {
                TT::Func | TT::Class | TT::Var | TT::For | TT::If | TT::Return => return,
                _ => self.advance(),
            };
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expression()
    }

    fn expression(&mut self) -> ParseResultFn {
        self.comma()
    }

    fn comma(&mut self) -> ParseResultFn {
        let mut expression = self.ternary()?;
        while self.match_token(&[TT::Comma]) {
            let right = self.ternary()?;
            expression = Expression::Comma {
                left: Box::new(expression),
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn ternary(&mut self) -> ParseResultFn {
        let mut expression = self.equality()?;
        if self.match_token(&[TT::Question]) {
            let t = self.expression()?;

            if self.match_token(&[TT::Colon]) {
                let f = self.ternary()?;
                expression = Expression::Ternary {
                    condition: Box::new(expression),
                    true_branch: Box::new(t),
                    false_branch: Box::new(f),
                };
            } else {
                return Err(LoxError::Parse {
                    message: String::from("expect ':' after ternary"),
                });
            };
        }

        Ok(expression)
    }

    fn equality(&mut self) -> ParseResultFn {
        let mut expression = self.comparison()?;
        while self.match_token(&[TT::EqualEqual, TT::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> ParseResultFn {
        let mut expression = self.term()?;
        while self.match_token(&[TT::Greater, TT::GreaterEqual, TT::Less, TT::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn term(&mut self) -> ParseResultFn {
        let mut expression = self.factor()?;
        while self.match_token(&[TT::Plus, TT::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn factor(&mut self) -> ParseResultFn {
        let mut expression = self.unary()?;
        while self.match_token(&[TT::Star, TT::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn unary(&mut self) -> ParseResultFn {
        if self.match_token(&[TT::Minus, TT::Bang]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        } else if self.match_token(&[
            TT::EqualEqual,
            TT::BangEqual,
            TT::Greater,
            TT::GreaterEqual,
            TT::Less,
            TT::LessEqual,
            TT::Plus,
            TT::Star,
            TT::Slash,
        ]) {
            return Err(LoxError::Parse {
                message: format!(
                    "missing left-hand operand before '{}'",
                    self.previous().lexeme
                ),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> ParseResultFn {
        let token = self.advance();
        match token.token_type {
            TT::True => Ok(Expression::Literal(LiteralValue::True)),
            TT::False => Ok(Expression::Literal(LiteralValue::False)),
            TT::Null => Ok(Expression::Literal(LiteralValue::Null)),
            TT::String => Ok(Expression::Literal(LiteralValue::String(
                token.lexeme.clone(),
            ))),
            TT::Number => {
                let num = token.lexeme.parse::<f64>();
                match num {
                    Ok(v) => Ok(Expression::Literal(LiteralValue::Number(v))),
                    Err(_) => Err(LoxError::Parse {
                        message: String::from("number is not in valid format"),
                    }),
                }
            }
            TT::LParen => {
                let expression = self.expression()?;
                if self.match_token(&[TT::RParen]) {
                    return Ok(Expression::Grouping(Box::new(expression)));
                }
                Err(LoxError::Parse {
                    message: String::from("expect ')' after expression"),
                })
            }
            _ => Err(LoxError::Parse {
                message: String::from("expect expression"),
            }),
        }
    }

    fn match_token(&mut self, tt: &[TT]) -> bool {
        for &t in tt {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tt: TT) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == tt
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TT::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
