use crate::{
    error::LoxError,
    expression::{self, Expression, LiteralValue},
    statement::Statement,
    token::{Token, TokenType as TT},
};

// TODO: remove `clone()`

// Redesign output strategy
pub type ParseResult = Result<Vec<Statement>, LoxError>;
type ParseStmtResultFn = Result<Statement, LoxError>;
type ParseExprResultFn = Result<Expression, LoxError>;

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
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> ParseStmtResultFn {
        if self.match_token(&[TT::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> ParseStmtResultFn {
        self.consume(TT::Identifier, String::from("expect variable name"))?;
        let name = self.previous().clone();

        let mut initializer = None;
        if self.match_token(&[TT::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TT::SemiColon,
            String::from("expect ';' after variable declaration"),
        )?;
        Ok(Statement::Var { name, initializer })
    }

    fn statement(&mut self) -> ParseStmtResultFn {
        if self.match_token(&[TT::Print]) {
            return self.print_statement();
        } else if self.match_token(&[TT::LBrace]) {
            return self.block_statement();
        } else if self.match_token(&[TT::If]) {
            return self.if_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> ParseStmtResultFn {
        let value = self.expression()?;
        self.consume(TT::SemiColon, String::from("expect ';' after value"))?;
        Ok(Statement::Print(value))
    }

    fn block_statement(&mut self) -> ParseStmtResultFn {
        let mut statements = vec![];

        while self.peek().token_type != TT::RBrace && !self.is_at_end() {
            let statement = self.declaration()?;
            statements.push(statement);
        }

        self.consume(TT::RBrace, String::from("expect '}' after block"))?;
        Ok(Statement::Block(statements))
    }

    fn if_statement(&mut self) -> ParseStmtResultFn {
        self.consume(TT::LParen, String::from("expect '(' after if statement"))?;
        let cond = self.expression()?;
        self.consume(
            TT::RParen,
            String::from("expect ')' after if statement condition"),
        )?;

        self.consume(
            TT::LBrace,
            String::from("expect '{' after if statement condition"),
        )?;

        let body = self.block_statement()?;
        let mut elze: Option<Box<Statement>> = None;
        if self.match_token(&[TT::Else]) {
            if self.match_token(&[TT::If]) {
                let stmt = self.if_statement()?;
                elze = Some(Box::new(stmt));
            } else {
                self.consume(TT::LBrace, String::from("expect '{' after 'else'"))?;
                let stmt = self.block_statement()?;
                elze = Some(Box::new(stmt));
            }
        };

        Ok(Statement::If {
            cond,
            body: Box::new(body),
            elze,
        })
    }

    fn expression_statement(&mut self) -> ParseStmtResultFn {
        let value = self.expression()?;
        self.consume(TT::SemiColon, String::from("expect ';' after value"))?;
        Ok(Statement::Expression(value))
    }

    fn expression(&mut self) -> ParseExprResultFn {
        self.comma()
    }

    fn comma(&mut self) -> ParseExprResultFn {
        let mut expression = self.assignment()?;
        while self.match_token(&[TT::Comma]) {
            let right = self.assignment()?;
            expression = Expression::Comma {
                left: Box::new(expression),
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn assignment(&mut self) -> ParseExprResultFn {
        let mut expression = self.ternary()?;
        if self.match_token(&[TT::Equal]) {
            let value = self.assignment()?;

            if let Expression::Variable(name) = expression {
                expression = Expression::Assignment {
                    name,
                    value: Box::new(value),
                }
            } else {
                return Err(LoxError::Parse {
                    message: String::from("invalid assignment target"),
                });
            }
        }

        Ok(expression)
    }

    fn ternary(&mut self) -> ParseExprResultFn {
        let mut expression = self.logical_or()?;
        if self.match_token(&[TT::Question]) {
            let t = self.expression()?;

            self.consume(TT::Colon, String::from("expect ':' after ternary"))?;
            let f = self.ternary()?;
            expression = Expression::Ternary {
                condition: Box::new(expression),
                true_branch: Box::new(t),
                false_branch: Box::new(f),
            };
        }

        Ok(expression)
    }

    fn logical_or(&mut self) -> ParseExprResultFn {
        let mut expression = self.logical_and()?;
        while self.match_token(&[TT::PipePipe]) {
            let op = self.previous().clone();
            let right = self.logical_and()?;

            expression = Expression::Logical {
                left: Box::new(expression),
                operator: op,
                right: Box::new(right),
            }
        }
        Ok(expression)
    }

    fn logical_and(&mut self) -> ParseExprResultFn {
        let mut expression = self.equality()?;
        while self.match_token(&[TT::AmpAmp]) {
            let op = self.previous().clone();
            let right = self.equality()?;

            expression = Expression::Logical {
                left: Box::new(expression),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn equality(&mut self) -> ParseExprResultFn {
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

    fn comparison(&mut self) -> ParseExprResultFn {
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

    fn term(&mut self) -> ParseExprResultFn {
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

    fn factor(&mut self) -> ParseExprResultFn {
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

    fn unary(&mut self) -> ParseExprResultFn {
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

    fn primary(&mut self) -> ParseExprResultFn {
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
                self.consume(TT::RParen, String::from("expect ')' after expression"))?;
                return Ok(Expression::Grouping(Box::new(expression)));
            }
            TT::Identifier => Ok(Expression::Variable(self.previous().clone())),
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

    fn consume(&mut self, token_type: TT, error_message: String) -> Result<&Token, LoxError> {
        if !self.match_token(&[token_type]) {
            return Err(LoxError::Parse {
                message: error_message,
            });
        }
        Ok(self.previous())
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

    fn expr_stmt(expr: Expression) -> Vec<Statement> {
        vec![Statement::Expression(expr)]
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
            }
            | Expression::Logical {
                left,
                operator,
                right,
            } => Expression::Binary {
                left: Box::new(normalize_expression(left)),
                operator: normalize_token(operator),
                right: Box::new(normalize_expression(right)),
            },
            Expression::Assignment { name, value } => Expression::Assignment {
                name: normalize_token(name),
                value: Box::new(normalize_expression(value)),
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
            Expression::Variable(var) => Expression::Variable(var.clone()),
        }
    }

    fn normalize_statement(statement: &Statement) -> Statement {
        match statement {
            Statement::If { cond, body, elze } => Statement::If {
                cond: normalize_expression(cond),
                body: Box::new(normalize_statement(body.as_ref())),
                elze: elze
                    .as_ref()
                    .map(|e| Box::new(normalize_statement(e.as_ref()))),
            },
            Statement::Var { name, initializer } => match initializer {
                Some(expr) => Statement::Var {
                    name: name.clone(),
                    initializer: Some(normalize_expression(expr)),
                },
                None => Statement::Var {
                    name: name.clone(),
                    initializer: None,
                },
            },
            Statement::Block(statements) => {
                let mut normalized_statements = vec![];
                for statement in statements {
                    let normalized_statement = normalize_statement(statement);
                    normalized_statements.push(normalized_statement);
                }
                Statement::Block(normalized_statements)
            }
            Statement::Expression(expression) => {
                Statement::Expression(normalize_expression(expression))
            }
            Statement::Print(expression) => Statement::Print(normalize_expression(expression)),
        }
    }

    fn normalize_statements(statements: Vec<Statement>) -> Vec<Statement> {
        let mut normalized = vec![];
        for statement in &statements {
            normalized.push(normalize_statement(statement));
        }
        normalized
    }

    fn get_parse_result(src: &str) -> Result<Vec<Statement>, LoxError> {
        let mut lexer = Lexer::new(src);
        let lex_result = lexer.lex_tokens();
        let mut parser = Parser::new(&lex_result.tokens);
        let parse_result = parser.parse()?;
        Ok(normalize_statements(parse_result))
    }

    #[test]
    fn arithmetic_operations() -> Result<(), LoxError> {
        let src = "\
        5+4*3-1/2;
        "
        .trim();

        let parse_result = get_parse_result(src)?;
        let expected = expr_stmt(binary(
            binary(num(5.0), TT::Plus, binary(num(4.0), TT::Star, num(3.0))),
            TT::Minus,
            binary(num(1.0), TT::Slash, num(2.0)),
        ));

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn unary_operations() -> Result<(), LoxError> {
        let src = "\
        --3!=!!!\"rlox\";
        ";

        let parse_result = get_parse_result(src)?;
        let expected = expr_stmt(binary(
            unary(TT::Minus, unary(TT::Minus, num(3.0))),
            TT::BangEqual,
            unary(
                TT::Bang,
                unary(TT::Bang, unary(TT::Bang, str("\"rlox\"".to_string()))),
            ),
        ));

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn ternary_operations() -> Result<(), LoxError> {
        let src = "\
        43.5 >= null ? -3 > 4 : !!true < 1 ? 0.0 <= false : 3 == \"rlox\";
        ";

        let parse_result = get_parse_result(src)?;
        let expected = expr_stmt(ternary(
            binary(num(43.5), TT::GreaterEqual, null()),
            binary(unary(TT::Minus, num(3.0)), TT::Greater, num(4.0)),
            ternary(
                binary(unary(TT::Bang, unary(TT::Bang, tr())), TT::Less, num(1.0)),
                binary(num(0.0), TT::LessEqual, fl()),
                binary(num(3.0), TT::EqualEqual, str("\"rlox\"".to_string())),
            ),
        ));

        assert_eq!(parse_result, expected);
        Ok(())
    }

    #[test]
    fn comma_operations() -> Result<(), LoxError> {
        let src = "\
        true ? 1 : 2, 3, 4;
        ";

        let parse_result = get_parse_result(src)?;
        let expected = expr_stmt(comma(
            comma(ternary(tr(), num(1.0), num(2.0)), num(3.0)),
            num(4.0),
        ));

        assert_eq!(parse_result, expected);
        Ok(())
    }
}
