use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    callable::{Callable, CallableKind},
    error::LoxError,
    function::Function,
    instance::Instance,
    interpreter::{Interpreter, Value},
};

#[derive(Clone)]
pub struct Class {
    name: String,
    superclass: Value,
    methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(name: String, superclass: Value, methods: HashMap<String, Function>) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: String) -> Option<Function> {
        if self.methods.contains_key(&name) {
            Some(self.methods.get(&name).expect("method must exist").clone())
        } else {
            match &self.superclass {
                Value::Class(class) => {
                    return class.find_method(name);
                }
                _ => {}
            };
            None
        }
    }
}

impl Callable for Class {
    fn get_kind(&self) -> CallableKind {
        CallableKind::Class
    }

    fn arity(&self) -> usize {
        let initializer_fn = self.find_method("init".to_string());
        if let Some(initializer) = initializer_fn {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let instance = Instance::new(self.clone());
        let instance = Rc::new(RefCell::new(instance));
        let initializer_fn = self.find_method("init".to_string());
        if let Some(initializer) = initializer_fn {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        };
        Ok(Value::Instance(instance))
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
