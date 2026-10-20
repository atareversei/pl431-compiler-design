// This is separate pass then the parser. "Crafting Interprets" points out that
// later in book this module will be placed in the parser.

use std::collections::HashMap;

use crate::{
    error::LoxError,
    expression::{Expression, ExpressionKind},
    interpreter::Interpreter,
    log,
    logger::{LogSection, Logger},
    statement::{FunctionStatement, Statement},
    token::Token,
};

type ResolveResultFn = Result<(), LoxError>;

pub struct Resolver<'resolver, 'interpreter> {
    interpreter: &'resolver mut Interpreter<'interpreter>,
    scopes: Vec<HashMap<String, bool>>,
    logger: &'resolver Logger,
}

impl<'resolver, 'interpreter> Resolver<'resolver, 'interpreter> {
    pub fn new(
        interpreter: &'resolver mut Interpreter<'interpreter>,
        logger: &'resolver Logger,
    ) -> Self {
        Self {
            interpreter,
            scopes: vec![HashMap::new()],
            logger,
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
                log!(self.logger, LogSection::ResolverFuncInfo, "Block");
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Statement::Var { name, initializer } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Var");
                self.declare(&name);
                if let Some(init) = &initializer {
                    self.resolve_expression(init)?;
                }
                self.define(&name);
            }
            Statement::Function(function) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Function");
                self.declare(&function.name);
                self.define(&function.name);

                self.resolve_function(function)?;
            }
            Statement::Class { name, methods } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Class");
                self.declare(name);
                self.define(name);
            }
            Statement::Expression(expression) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Expression");
                self.resolve_expression(expression)?;
            }
            Statement::If { cond, body, elze } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "If");
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
                if let Some(el) = elze {
                    self.resolve_statement(el)?;
                }
            }
            Statement::Print(expression) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Print");
                self.resolve_expression(expression)?;
            }
            Statement::Return(expression) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Return");
                if let Some(expr) = expression {
                    self.resolve_expression(expr)?;
                }
            }
            Statement::For {
                increment,
                cond,
                body,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "For");
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
                log!(self.logger, LogSection::ResolverFuncInfo, "Variable");

                if self
                    .scopes
                    .last()
                    .and_then(|scope| scope.get(&variable.lexeme))
                    == Some(&false)
                {
                    return Err(LoxError::Parse {
                        message: String::from("cannot read variable inside its own initializer"),
                    });
                }
                self.resolve_local(expression, variable);
            }
            ExpressionKind::Assignment { name, value } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Assignment");
                self.resolve_expression(value)?;
                self.resolve_local(expression, name);
            }
            ExpressionKind::Binary {
                operator: _,
                left,
                right,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Binary");
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Call {
                paren: _,
                callee,
                arguments,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Call");
                self.resolve_expression(callee)?;
                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
            ExpressionKind::Grouping(expression) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Grouping");
                self.resolve_expression(expression)?;
            }
            ExpressionKind::Logical {
                operator: _,
                left,
                right,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Logical");
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Unary { operator: _, right } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Unary");
                self.resolve_expression(right)?;
            }
            ExpressionKind::Comma { left, right } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Comma");
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            ExpressionKind::Ternary {
                condition,
                true_branch,
                false_branch,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Ternary");
                self.resolve_expression(condition)?;
                self.resolve_expression(true_branch)?;
                self.resolve_expression(false_branch)?;
            }

            ExpressionKind::Literal(_) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Literal");
            }
            _ => {}
        }
        Ok(())
        // TODO: implement
    }

    fn resolve_function(&mut self, function: &FunctionStatement) -> ResolveResultFn {
        log!(
            self.logger,
            LogSection::ResolverFuncInfo,
            "resolve_function"
        );
        self.begin_scope();
        for param in &function.parameters {
            self.declare(&param);
            self.define(&param);
        }
        if let Some(body) = &function.body {
            match body.as_ref() {
                Statement::Block(statements) => {
                    self.resolve(statements)?;
                }
                other => {
                    self.resolve_statement(other)?;
                }
            };
        }
        self.end_scope();
        Ok(())
    }

    fn resolve_local(&mut self, expression: &Expression, name: &Token) {
        log!(self.logger, LogSection::ResolverFuncInfo, "resolve_local");
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) || i == 0 {
                self.interpreter
                    .resolve(expression, self.scopes.len() - 1 - i);
                break;
            }
        }
    }

    fn begin_scope(&mut self) {
        log!(self.logger, LogSection::ResolverFuncInfo, "begin_scope");
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        log!(self.logger, LogSection::ResolverFuncInfo, "end_scope");
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        log!(self.logger, LogSection::ResolverFuncInfo, "declare");
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut();
        if let Some(s) = scope {
            s.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        log!(self.logger, LogSection::ResolverFuncInfo, "define");
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut();
        if let Some(s) = scope {
            s.insert(name.lexeme.clone(), true);
        }
    }
}
