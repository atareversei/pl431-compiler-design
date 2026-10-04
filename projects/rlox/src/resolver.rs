// This is separate pass then the parser. "Crafting Interprets" points out that
// later in book this module will be placed in the parser.

use std::collections::HashMap;

use crate::{
    expression::Expression,
    interpreter::{self, Interpreter},
    statement::Statement,
    token::Token,
};

pub struct Resolver<'a> {
    interpreter: Interpreter<'a>,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: Interpreter<'a>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    fn resolve_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Block(statements) => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
            }
            Statement::Var { name, initializer } => {
                self.declare(&name);
                if let Some(init) = initializer {
                    self.resolve_expression(init);
                }
                self.define(&name);
            }
            _ => {}
        }
    }

    fn resolve_expression(&self, expression: Expression) {
        // TODO: implement
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
