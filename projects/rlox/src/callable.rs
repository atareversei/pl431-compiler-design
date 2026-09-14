use crate::interpreter::Value;

pub trait Callable {
    fn call(&self, arguments: Vec<Value>) {}
}
