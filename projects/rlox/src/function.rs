use crate::{
    callable::Callable,
    environment::{self, Environment},
    error::LoxError,
    interpreter::{Interpreter, Value},
    statement::FunctionStatement,
};

pub struct Function {
    declaration: FunctionStatement,
}

impl Function {
    pub fn new(declaration: FunctionStatement) -> Self {
        Self { declaration }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let mut environment = Environment::globals();
        for (param, argument) in self.declaration.parameters.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), argument);
        }

        let Some(body) = &self.declaration.body else {
            return Ok(Value::Null);
        };

        let result = interpreter.execute_statement(body.as_ref(), Some(environment))?;

        Ok(result.unwrap_or(Value::Null))
    }
}
