use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    callable::Callable,
    error::LoxError,
    interpreter::{Interpreter, Value},
};

pub struct Clock;

impl Callable for Clock {
    fn name(&self) -> String {
        "<native> clock".to_string()
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(value) => Ok(Value::Number(value.as_secs_f64())),
            Err(_) => Err(LoxError::Runtime {
                message: String::from("system clock is before unix epoch"),
            }),
        }
    }
}
