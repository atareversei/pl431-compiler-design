use crate::{
    error::LoxError,
    interpreter::{Interpreter, Value},
};


pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>)
    -> Result<Value, LoxError>;
}
