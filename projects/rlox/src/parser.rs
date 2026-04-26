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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenType as TT};

    fn tr() -> Expression {
        Expression::Literal(LiteralValue::True)
    }

    fn fl() -> Expression {
        Expression::Literal(LiteralValue::False)
    }

    fn null() -> Expression {
        Expression::Literal(LiteralValue::Null)
    }

    fn num(n: f64) -> Expression {
        Expression::Literal(LiteralValue::Number(n))
    }

    fn str(s: String) -> Expression {
        Expression::Literal(LiteralValue::String(s))
    }

    fn binary(left: Expression, operator: TT, right: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(left),
            operator: Token::new(operator, 0, 0, operator.test_lexeme().to_string()),
            right: Box::new(right),
        }
    }

    fn unary(operator: TT, right: Expression) -> Expression {
        Expression::Unary {
            operator: Token::new(operator, 0, 0, operator.test_lexeme().to_string()),
            right: Box::new(right),
        }
    }

    fn ternary(
        condition: Expression,
        true_branch: Expression,
        false_branch: Expression,
    ) -> Expression {
        Expression::Ternary {
            condition: Box::new(condition),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        }
    }

    fn comma(left: Expression, right: Expression) -> Expression {
        Expression::Comma {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn normalize_token(token: &Token) -> Token {
        Token::new(
            token.token_type,
            0,
            0,
            token.token_type.test_lexeme().to_string(),
        )
    }

    fn normalize_expression(expression: &Expression) -> Expression {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => Expression::Binary {
                left: Box::new(normalize_expression(left)),
                operator: normalize_token(operator),
                right: Box::new(normalize_expression(right)),
            },
            Expression::Unary { operator, right } => Expression::Unary {
                operator: normalize_token(operator),
                right: Box::new(normalize_expression(right)),
            },
            Expression::Literal(lit) => Expression::Literal(lit.clone()),
            Expression::Grouping(inner) => {
                Expression::Grouping(Box::new(normalize_expression(inner)))
            }
            Expression::Ternary {
                condition,
                true_branch,
                false_branch,
            } => Expression::Ternary {
                condition: Box::new(normalize_expression(condition)),
                true_branch: Box::new(normalize_expression(true_branch)),
                false_branch: Box::new(normalize_expression(false_branch)),
            },
            Expression::Comma { left, right } => Expression::Comma {
                left: Box::new(normalize_expression(left)),
                right: Box::new(normalize_expression(right)),
            },
        }
    }

    fn get_parse_result(src: &str) -> Result<Expression, LoxError> {
        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();
        let mut parser = Parser::new(&lex_result.tokens);
        let parse_result = parser.parse()?;
        Ok(normalize_expression(&parse_result))
    }

    #[test]
    fn arithmetic_operations() -> Result<(), LoxError> {
        let src = "\
        5+4*3-1/2
        "
        .trim();

        let parse_result = get_parse_result(src)?;
        let expected = binary(
            binary(num(5.0), TT::Plus, binary(num(4.0), TT::Star, num(3.0))),
            TT::Minus,
            binary(num(1.0), TT::Slash, num(2.0)),
        );

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn unary_operations() -> Result<(), LoxError> {
        let src = "\
        --3!=!!!\"rlox\"
        ";

        let parse_result = get_parse_result(src)?;
        let expected = binary(
            unary(TT::Minus, unary(TT::Minus, num(3.0))),
            TT::BangEqual,
            unary(
                TT::Bang,
                unary(TT::Bang, unary(TT::Bang, str("\"rlox\"".to_string()))),
            ),
        );

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn ternary_operations() -> Result<(), LoxError> {
        let src = "\
        43.5 >= null ? -3 > 4 : !!true < 1 ? 0.0 <= false : 3 == \"rlox\" : 
        ";

        let parse_result = get_parse_result(src)?;
        let expected = ternary(
            binary(num(43.5), TT::GreaterEqual, null()),
            binary(unary(TT::Minus, num(3.0)), TT::Greater, num(4.0)),
            ternary(
                binary(unary(TT::Bang, unary(TT::Bang, tr())), TT::Less, num(1.0)),
                binary(num(0.0), TT::LessEqual, fl()),
                binary(num(3.0), TT::EqualEqual, str("\"rlox\"".to_string())),
            ),
        );

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn comma_operations() -> Result<(), LoxError> {
        let src = "\
        true ? 1 : 2, 3, 4
        ";

        let parse_result = get_parse_result(src)?;
        let expected = comma(comma(ternary(tr(), num(1.0), num(2.0)), num(3.0)), num(4.0));

        assert_eq!(parse_result, expected);
        Ok(())
    }
}
