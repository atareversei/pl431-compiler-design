use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{error::LoxError, interpreter::Value};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), LoxError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(LoxError::Runtime {
                    message: format!("undefined variable: '{}'", name),
                }),
            }
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, LoxError> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().get(name)
        } else {
            Err(LoxError::Runtime {
                message: format!("undefined variable: {}", name),
            })
        }
    }
}
