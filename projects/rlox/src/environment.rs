use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{environment, error::LoxError, interpreter::Value, native::clock::Clock};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn globals() -> Self {
        let mut values = HashMap::new();

        values.insert("clock".to_string(), Value::Callable(Rc::new(Clock)));

        Environment {
            enclosing: None,
            values,
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn enclose_by(mut self, enclosing: Rc<RefCell<Environment>>) -> Self {
        self.enclosing = Some(enclosing);
        self
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

    pub fn assign_at(
        environment: &Rc<RefCell<Environment>>,
        distance: usize,
        name: &str,
        value: Value,
    ) -> Result<(), LoxError> {
        let mut env = Rc::clone(environment);
        for i in 0..distance {
            let enclosing = env
                .borrow()
                .enclosing
                .clone()
                .ok_or_else(|| LoxError::Runtime {
                    message: String::from("no enclosing environment found"),
                })?;
            env = enclosing;
        }

        if env.borrow_mut().values.contains_key(name) {
            env.borrow_mut().values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(LoxError::Runtime {
                message: format!("undefined variable: '{}'", name),
            })
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

    pub fn get_at(
        environment: &Rc<RefCell<Environment>>,
        distance: usize,
        name: &str,
    ) -> Result<Value, LoxError> {
        let mut env = Rc::clone(environment);
        for i in 0..distance {
            let enclosing = env
                .borrow()
                .enclosing
                .clone()
                .ok_or_else(|| LoxError::Runtime {
                    message: String::from("no enclosing environment found"),
                })?;
            env = enclosing;
        }

        if let Some(value) = env.borrow().values.get(name) {
            Ok(value.clone())
        } else {
            Err(LoxError::Runtime {
                message: format!("undefined variable: {}", name),
            })
        }
    }
}
