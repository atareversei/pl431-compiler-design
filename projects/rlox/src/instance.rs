use std::collections::HashMap;

use crate::{class::Class, error::LoxError, interpreter::Value};

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

    pub fn get(&self, name: String) -> Result<&Value, LoxError> {
        if self.fields.contains_key(&name) {
            Ok(self.fields.get(&name).expect("field exists"))
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
