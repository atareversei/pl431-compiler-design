use crate::{
    error::LoxError,
    interpreter::{Interpreter, Value},
};

pub enum CallableKind {
    Function,
    Class,
}

pub trait Callable {
    fn get_kind(&self) -> CallableKind;
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>)
    -> Result<Value, LoxError>;
}
