use crate::{
    error::LoxError,
    expression::{Expression, LiteralValue},
    statement::Statement,
    token::TokenType as TT,
};

pub type ExecutionResult = Result<(), LoxError>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
pub type EvaluationResult = Result<Value, LoxError>;

pub struct Interpreter {
    statements: Vec<Statement>,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Self {
        Interpreter { statements }
    }

    pub fn interpret(&self) -> ExecutionResult {
        for statement in &self.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    pub fn execute_statement(&self, statement: &Statement) -> ExecutionResult {
        match statement {
            Statement::Expression(expression) => {
                self.evaluate_expression(expression)?;
                Ok(())
            }
            Statement::Print(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{:?}", value);
                Ok(())
            }
        }
    }

    pub fn evaluate_expression(&self, expression: &Expression) -> EvaluationResult {
        match expression {
            Expression::Ternary {
                condition,
                true_branch,
                false_branch,
            } => {
                if self.is_truthy(self.evaluate_expression(condition)?) {
                    self.evaluate_expression(true_branch)
                } else {
                    self.evaluate_expression(false_branch)
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;

                match operator.token_type {
                    TT::EqualEqual => Ok(Value::Boolean(self.is_equal(left_value, right_value))),

                    TT::BangEqual => Ok(Value::Boolean(!self.is_equal(left_value, right_value))),

                    TT::Greater => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Boolean(l > r))
                    }
                    TT::GreaterEqual => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Boolean(l >= r))
                    }
                    TT::Less => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Boolean(l < r))
                    }
                    TT::LessEqual => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Boolean(l <= r))
                    }

                    TT::Plus => match (left_value, right_value) {
                        (Value::String(l), Value::String(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }
                        (Value::Number(l), Value::String(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }
                        (Value::String(l), Value::Number(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }

                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        _ => Err(LoxError::Runtime {
                            message: String::from("operands must be two numbers or two strings"),
                        }),
                    },
                    TT::Minus => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Number(l - r))
                    }
                    TT::Star => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        Ok(Value::Number(l * r))
                    }
                    TT::Slash => {
                        let (l, r) = self.check_number_operands(
                            operator.lexeme.to_string(),
                            left_value,
                            right_value,
                        )?;
                        if r == 0.0 {
                            return Err(LoxError::Runtime {
                                message: String::from("division by 0 is not allowed"),
                            });
                        }

                        Ok(Value::Number(l / r))
                    }
                    _ => unreachable!("binary only allows '+', '-', '*', and '/' as operators"),
                }
            }
            Expression::Unary { operator, right } => {
                let right_value = self.evaluate_expression(right)?;

                match operator.token_type {
                    TT::Minus => {
                        if let Value::Number(n) = right_value {
                            Ok(Value::Number(-n))
                        } else {
                            Err(LoxError::Runtime {
                                message: String::from("operand must be a number"),
                            })
                        }
                    }
                    TT::Bang => Ok(Value::Boolean(!self.is_truthy(right_value))),
                    _ => unreachable!("unary only allows '-' and '!' as operators"),
                }
            }
            Expression::Comma { left, right } => {
                self.evaluate_expression(left)?;
                self.evaluate_expression(right)
            }
            Expression::Grouping(expr) => self.evaluate_expression(expr),
            Expression::Literal(value) => match value {
                LiteralValue::False => Ok(Value::Boolean(false)),
                LiteralValue::True => Ok(Value::Boolean(true)),
                LiteralValue::Number(n) => Ok(Value::Number(*n)),
                LiteralValue::String(s) => Ok(Value::String(s.clone())),
                LiteralValue::Null => Ok(Value::Null),
            },
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Null => false,
            Value::Boolean(b) => b,
            _ => true,
        }
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        match (&a, &b) {
            (Value::Null, Value::Null) => true,
            (Value::Null, _) => false,
            _ => a == b,
        }
    }

    fn check_number_operands(
        &self,
        operator: String,
        left: Value,
        right: Value,
    ) -> Result<(f64, f64), LoxError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok((l, r)),
            (Value::Number(_), _) => Err(LoxError::Runtime {
                message: format!("right operand must be a number for '{}'", operator),
            }),
            (_, Value::Number(_)) => Err(LoxError::Runtime {
                message: format!("left operand must be a number for '{}'", operator),
            }),
            _ => Err(LoxError::Runtime {
                message: format!("both operands must be a number for '{}'", operator),
            }),
        }
    }
}
