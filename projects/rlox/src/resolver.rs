// This is separate pass then the parser. "Crafting Interprets" points out that
// later in book this module will be placed in the parser.

use std::collections::HashMap;

use crate::{
    error::LoxError,
    expression::{Expression, ExpressionKind},
    interpreter::Interpreter,
    statement::{FunctionStatement, Statement},
    token::Token,
};

type ResolveResultFn = Result<(), LoxError>;

pub struct Resolver<'resolver, 'interpreter> {
    interpreter: &'resolver mut Interpreter<'interpreter>,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'resolver, 'interpreter> Resolver<'resolver, 'interpreter> {
    pub fn new(interpreter: &'resolver mut Interpreter<'interpreter>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Statement>) -> ResolveResultFn {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Statement) -> ResolveResultFn {
        match statement {
            Statement::Block(statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Statement::Var { name, initializer } => {
                self.declare(&name);
                if let Some(init) = &initializer {
                    self.resolve_expression(init)?;
                }
                self.define(&name);
            }
            Statement::Function(function) => {
                self.declare(&function.name);
                self.define(&function.name);

                self.resolve_function(function);
            }
            Statement::Expression(expression) => {
                self.resolve_expression(expression);
            }
            Statement::If { cond, body, elze } => {
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
                if let Some(el) = elze {
                    self.resolve_statement(el)?;
                }
            }
            Statement::Print(expression) => {
                self.resolve_expression(expression)?;
            }
            Statement::Return(expression) => {
                if let Some(expr) = expression {
                    self.resolve_expression(expr)?;
                }
            }
            Statement::For {
                increment,
                cond,
                body,
            } => {
                if let Some(inc) = increment {
                    self.resolve_expression(inc)?;
                };
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
            }
            Statement::Break | Statement::Continue => {}
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expression: &Expression) -> ResolveResultFn {
        match &expression.kind {
            ExpressionKind::Variable(variable) => {
                if self
                    .scopes
                    .last()
                    .and_then(|scope| scope.get(&variable.lexeme))
                    == Some(&false)
                {
                    return Err(LoxError::Parse {
                        message: String::from(""),
                    });
                }
                self.resolve_local(expression, variable);
            }
            ExpressionKind::Assignment { name, value } => {
                self.resolve_expression(value)?;
                self.resolve_local(expression, name);
            }
            ExpressionKind::Binary {
                operator: _,
                left,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Call {
                paren: _,
                callee,
                arguments,
            } => {
                self.resolve_expression(callee)?;
                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
            ExpressionKind::Grouping(expression) => {
                self.resolve_expression(expression)?;
            }
            ExpressionKind::Logical {
                operator: _,
                left,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Unary { operator: _, right } => {
                self.resolve_expression(right)?;
            }
            ExpressionKind::Comma { left, right } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Ternary {
                condition,
                true_branch,
                false_branch,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_expression(true_branch)?;
                self.resolve_expression(false_branch)?;
            }

            ExpressionKind::Literal(_) => {}
            _ => {}
        }
        Ok(())
        // TODO: implement
    }

    fn resolve_function(&mut self, function: &FunctionStatement) -> ResolveResultFn {
        self.begin_scope();
        for param in &function.parameters {
            self.declare(&param);
            self.define(&param);
        }
        if let Some(body) = &function.body {
            self.resolve_statement(body.as_ref())?;
        }
        self.end_scope();
        Ok(())
    }

    fn resolve_local(&mut self, expression: &Expression, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expression, self.scopes.len() - 1 - i);
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut();
        if let Some(s) = scope {
            s.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut();
        if let Some(s) = scope {
            s.insert(name.lexeme.clone(), true);
        }
    }
}
