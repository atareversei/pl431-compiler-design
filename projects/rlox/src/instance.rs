use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{class::Class, error::LoxError, interpreter::Value};

#[derive(Clone)]
pub struct Instance {
    class: Class,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(self, name: String) -> Result<Value, LoxError> {
        if self.fields.contains_key(&name) {
            Ok(self.fields.get(&name).expect("field exists").clone())
        } else if let Some(method) = self.class.find_method(name.clone()) {
            Ok(Value::Callable(Rc::new(
                method.bind(Rc::new(RefCell::new(self))),
            )))
        } else {
            Err(LoxError::Runtime {
                message: format!("undefined property: {}", name),
            })
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }
}
