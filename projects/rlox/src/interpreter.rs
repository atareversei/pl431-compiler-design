use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::LoxError,
    expression::{Expression, LiteralValue},
    statement::Statement,
    token::TokenType as TT,
};

pub struct ExecutionContext {
    pub environment: Rc<RefCell<Environment>>,
    pub last_expr_value: Option<Value>,
}
pub type ExecutionResult = Result<Option<Value>, LoxError>;

// TODO: check to see if Value and LiteralValue could be merged into one entity
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
pub type EvaluationResult = Result<Value, LoxError>;

enum LoopFlow {
    Normal,
    Break,
    Continue,
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    statements: Vec<Statement>,
    loop_depth: usize,
    loop_flow: LoopFlow,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>, environment: Environment) -> Self {
        Interpreter {
            statements,
            environment: Rc::new(RefCell::new(environment)),
            loop_depth: 0,
            loop_flow: LoopFlow::Normal,
        }
    }

    // TODO: read more about Rust best practice and rewrite this function
    pub fn interpret(&mut self) -> Result<ExecutionContext, LoxError> {
        let mut value = None;
        let statements = self.statements.clone();

        for statement in &statements {
            value = self.execute_statement(statement)?;
        }

        Ok(ExecutionContext {
            environment: self.environment.clone(),
            last_expr_value: value,
        })
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> ExecutionResult {
        match statement {
            Statement::If { cond, body, elze } => {
                let cond = self.evaluate_expression(cond)?;
                let cond = self.is_truthy(cond);
                if cond {
                    self.execute_statement(body)?;
                } else if let Some(e) = elze.as_ref() {
                    self.execute_statement(e.as_ref())?;
                }
                Ok(None)
            }
            Statement::For {
                increment,
                cond,
                body,
            } => {
                self.loop_depth += 1;
                loop {
                    let cond = self.evaluate_expression(cond)?;
                    let cond = self.is_truthy(cond);
                    if cond {
                        self.execute_statement(body)?;

                        match self.loop_flow {
                            LoopFlow::Break => {
                                self.loop_flow = LoopFlow::Normal;
                                break;
                            }
                            LoopFlow::Continue => {
                                self.loop_flow = LoopFlow::Normal;
                            }
                            _ => {}
                        };

                        if let Some(inc) = increment {
                            self.evaluate_expression(inc)?;
                        }
                    } else {
                        break;
                    }
                }
                self.loop_depth -= 1;
                Ok(None)
            }
            Statement::Break => {
                if self.loop_depth == 0 {
                    return Err(LoxError::Runtime {
                        message: String::from("cannot use 'break' outside of loop body"),
                    });
                }
                self.loop_flow = LoopFlow::Break;

                Ok(None)
            }
            Statement::Continue => {
                if self.loop_depth == 0 {
                    return Err(LoxError::Runtime {
                        message: String::from("cannot use 'continue' outside of loop body"),
                    });
                }
                self.loop_flow = LoopFlow::Continue;
                Ok(None)
            }
            Statement::Var { name, initializer } => {
                let mut value = Value::Null;
                // handle uninitialized variable evaluation
                if initializer.is_none() {
                    return Err(LoxError::Runtime {
                        message: format!(
                            "variable {} accessed without assigning any value to it",
                            name.lexeme
                        ),
                    });
                };

                // handle value evaluation if value is not set to LiteralValue::Null
                if &Some(Expression::Literal(LiteralValue::Null)) != initializer {
                    value = self.evaluate_expression(
                        initializer
                            .as_ref()
                            .expect("expected an expression other than null"),
                    )?;
                }

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
                Ok(None)
            }
            Statement::Block(statements) => {
                let local_env = Environment::new_enclosed(self.environment.clone());
                let local_env_rc = Rc::new(RefCell::new(local_env));

                let prev_env = std::mem::replace(&mut self.environment, local_env_rc);

                for statement in statements {
                    self.execute_statement(statement)?;
                    match self.loop_flow {
                        LoopFlow::Break | LoopFlow::Continue => break,
                        _ => {}
                    }
                }

                self.environment = prev_env;
                Ok(None)
            }
            Statement::Expression(expression) => {
                let value = self.evaluate_expression(expression)?;
                Ok(Some(value))
            }
            Statement::Print(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{:?}", value);
                Ok(None)
            }
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> EvaluationResult {
        match expression {
            Expression::Ternary {
                condition,
                true_branch,
                false_branch,
            } => {
                let condition = self.evaluate_expression(condition)?;
                if self.is_truthy(condition) {
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
            Expression::Assignment { name, value } => {
                let value = self.evaluate_expression(value)?;
                self.environment
                    .borrow_mut()
                    .assign(&name.lexeme, value.clone())?;
                Ok(value)
            }
            Expression::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate_expression(left)?;
                if operator.token_type == TT::PipePipe && self.is_truthy(left.clone()) {
                    return Ok(left);
                }
                if operator.token_type == TT::AmpAmp && !self.is_truthy(left.clone()) {
                    return Ok(left);
                }
                let right = self.evaluate_expression(right)?;
                Ok(right)
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
            Expression::Variable(name) => self.environment.borrow_mut().get(&name.lexeme),
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

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    fn get_test_file_path(filename: &str) -> PathBuf {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set");

        PathBuf::from(manifest_dir)
            .join("tests")
            .join("examples")
            .join(filename)
    }

    fn get_result(path: &str) -> Result<(), LoxError> {
        let path = get_test_file_path(path);
        let bytes = fs::read(&path).map_err(|err| LoxError::Scan {
            message: format!(
                "couldn't read file: /tests/examples/{} - error: {err}",
                path.display()
            ),
        })?;
        let src = String::from_utf8_lossy(&bytes);
        let mut lexer = Lexer::new(&src);
        let lex_result = lexer.lex_tokens();
        let mut parser = Parser::new(&lex_result.tokens);
        let parse_result = parser.parse()?;
        let environment = Environment::new();
        let mut interpreter = Interpreter::new(parse_result, environment);
        interpreter.interpret()?;
        Ok(())
    }

    #[test]
    fn block_and_scope() {
        let result = get_result("block.rlox");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn uninitialized_variable() {
        let result = get_result("uninitialized.error.rlox");
        assert!(matches!(result, Err(LoxError::Runtime { .. })));
    }

    #[test]
    fn loops() {
        let result = get_result("loop.rlox");
        assert_eq!(result, Ok(()));
    }
}
