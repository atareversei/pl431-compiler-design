use std::collections::HashMap;

use crate::{error::LoxError, interpreter::Value};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Value, LoxError> {
        self.values
            .get(&name)
            .cloned()
            .ok_or_else(|| LoxError::Runtime {
                message: format!("undefined variable '{}'", name),
            })
    }
}
