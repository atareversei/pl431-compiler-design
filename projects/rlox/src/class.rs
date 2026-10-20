use std::rc::Rc;

use crate::{
    callable::Callable,
    error::LoxError,
    instance::Instance,
    interpreter::{Interpreter, Value},
};

#[derive(Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let instance = Instance::new(self.clone());
        Ok(Value::Instance(Rc::new(instance)))
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
