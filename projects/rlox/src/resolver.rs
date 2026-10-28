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

#[derive(Clone, PartialEq)]
enum FunctionKind {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Clone, PartialEq)]
enum ClassKind {
    None,
    Class,
    SubClass,
}

pub struct Resolver<'resolver, 'interpreter> {
    interpreter: &'resolver mut Interpreter<'interpreter>,
    scopes: Vec<HashMap<String, bool>>,
    curr_func_kind: FunctionKind,
    curr_class_kind: ClassKind,
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
            curr_func_kind: FunctionKind::None,
            curr_class_kind: ClassKind::None,
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

                self.resolve_function(function, FunctionKind::Function)?;
            }
            Statement::Class {
                name,
                superclass,
                methods,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Class");
                let enclosing_class_kind = self.curr_class_kind.clone();
                self.curr_class_kind = ClassKind::Class;
                self.declare(name);
                self.define(name);

                if let Some(superclass) = superclass {
                    match &superclass.kind {
                        ExpressionKind::Variable(token) => {
                            if token.lexeme.clone() == name.lexeme.clone() {
                                return Err(LoxError::Runtime {
                                    message: String::from("a class cannot inherit itself"),
                                });
                            } else {
                                self.resolve_expression(superclass)?;
                                self.curr_class_kind = ClassKind::SubClass;
                                self.begin_scope();
                                self.scopes
                                    .last_mut()
                                    .expect("a scope has begun")
                                    .insert(String::from("super"), true);
                            }
                        }

                        _ => {
                            return Err(LoxError::Runtime {
                                message: String::from("superclass must be an identifier"),
                            });
                        }
                    };
                }

                self.begin_scope();
                self.scopes
                    .last_mut()
                    .expect("a scope has begun")
                    .insert(String::from("this"), true);
                for method in methods {
                    let mut declaration = FunctionKind::Method;
                    if method.name.lexeme.eq("init") {
                        declaration = FunctionKind::Initializer;
                    }
                    self.resolve_function(method, declaration)?;
                }
                self.end_scope();

                if let Some(superclass) = superclass {
                    match &superclass.kind {
                        ExpressionKind::Variable(token) => {
                            if token.lexeme.clone() == name.lexeme.clone() {
                            } else {
                                self.end_scope();
                            }
                        }

                        _ => {}
                    };
                }

                self.curr_class_kind = enclosing_class_kind;
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
                // TODO: interpreter is doing a similar check, remove that.
                if self.curr_func_kind == FunctionKind::None {
                    return Err(LoxError::Parse {
                        message: String::from("can't return from top-level code"),
                    });
                }
                if let Some(expr) = expression {
                    if self.curr_func_kind == FunctionKind::Initializer {
                        return Err(LoxError::Runtime {
                            message: String::from("cannot return value from an initializer"),
                        });
                    }
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
            ExpressionKind::Set {
                object,
                name,
                value,
            } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Set");
                self.resolve_expression(object)?;
                self.resolve_expression(value)?;
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

            ExpressionKind::Get { object, name } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Get");
                self.resolve_expression(object)?;
            }

            ExpressionKind::Literal(_) => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Literal");
            }
            ExpressionKind::Super { keyword, method } => {
                log!(self.logger, LogSection::ResolverFuncInfo, "Super");
                match &self.curr_class_kind {
                    ClassKind::None => {
                        return Err(LoxError::Parse {
                            message: String::from("can't use 'super' out of a class"),
                        });
                    }
                    ClassKind::Class => {
                        return Err(LoxError::Parse {
                            message: String::from(
                                "can't use 'super' in a calss with no super class",
                            ),
                        });
                    }
                    ClassKind::SubClass => {}
                };
                self.resolve_local(expression, keyword);
            }
            ExpressionKind::This(token) => {
                if self.curr_class_kind == ClassKind::None {
                    return Err(LoxError::Runtime {
                        message: String::from("cannot use 'this' outside of a class"),
                    });
                };
                self.resolve_local(expression, token);
            }
        }
        Ok(())
    }

    fn resolve_function(
        &mut self,
        function: &FunctionStatement,
        kind: FunctionKind,
    ) -> ResolveResultFn {
        log!(
            self.logger,
            LogSection::ResolverFuncInfo,
            "resolve_function"
        );
        let enclosingFunction = self.curr_func_kind.clone();
        self.curr_func_kind = kind;
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
        self.curr_func_kind = enclosingFunction;
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
