use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::Callable,
    environment::{self, Environment},
    error::LoxError,
    interpreter::{Interpreter, Value},
    statement::FunctionStatement,
};

pub struct Function {
    declaration: FunctionStatement,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: FunctionStatement, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn name(&self) -> String {
        self.declaration.name.to_string()
    }

    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let mut environment = Environment::new_enclosed(self.closure.clone());
        for (param, argument) in self.declaration.parameters.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), argument);
        }

        let Some(body) = &self.declaration.body else {
            return Ok(Value::Null);
        };

        interpreter.execute_statement(body.as_ref(), Some(environment))?;
        let value = interpreter
            .function_stack
            .last()
            .map_or(Value::Null, |f| f.returned.clone());

        Ok(value)
    }
}
