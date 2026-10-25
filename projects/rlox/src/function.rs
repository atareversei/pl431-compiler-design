use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::Callable,
    environment::{self, Environment},
    error::LoxError,
    instance::Instance,
    interpreter::{Interpreter, Value},
    statement::FunctionStatement,
};

#[derive(Clone)]
pub struct Function {
    declaration: FunctionStatement,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl Function {
    pub fn new(
        declaration: FunctionStatement,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }
}

impl Function {
    pub fn bind(&self, instance: Rc<RefCell<Instance>>) -> Self {
        let mut environment = Environment::new_enclosed(self.closure.clone());
        environment.define("this".to_string(), Value::Instance(instance));
        Function {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
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
        let value = interpreter.function_stack.last().map_or_else(
            || {
                if self.is_initializer {
                    return Environment::get_at(&self.closure, 0, "this");
                }
                Ok(Value::Null)
            },
            |f| Ok(f.returned.clone()),
        )?;

        if self.is_initializer {
            return Environment::get_at(&self.closure, 0, "this");
        }
        Ok(value)
    }
}
