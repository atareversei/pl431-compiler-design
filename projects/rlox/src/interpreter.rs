use crate::error::LoxError;
use crate::expression::{Expression, LiteralValue};
use crate::token::TokenType as TT;

#[derive(Debug)]
enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub type InterpretResult = Result<Value, LoxError>;
type InterpretResultFn = Result<Value, LoxError>;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expression: &Expression) -> InterpretResult {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.interpret(left)?;
                let right_value = self.interpret(right)?;

                match operator.token_type {
                    TT::Plus => {}
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
                        Ok(Value::Number(l / r))
                    }
                    _ => unreachable!("binary only allows '+', '-', '*', and '/' as operators"),
                }
            }
            Expression::Unary { operator, right } => {
                let right_value = self.interpret(right)?;

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
            Expression::Grouping(expr) => self.interpret(expr),
            Expression::Literal(value) => match value {
                LiteralValue::False => Ok(Value::Boolean(false)),
                LiteralValue::True => Ok(Value::Boolean(true)),
                LiteralValue::Number(n) => Ok(Value::Number(*n)),
                LiteralValue::String(s) => Ok(Value::String(*s)),
                LiteralValue::Null => Ok(Value::Null),
            },
            _ => Ok(Value::Null),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Null => false,
            Value::Boolean(b) => b,
            _ => true,
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
