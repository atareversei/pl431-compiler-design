use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// This code is experimental in Rust
// and the semantics are already implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    // `get` never gives out a pointer, it returns a copy of the value.
    // This helps us to avoid codes like this:
    //
    // let x = Cell::new(vec![42]);
    // let first = &x.get()[0];
    // x.set(vec![]);
    // * at this point first should have been invalidated if we were modifying the same
    // * underlying data, but in fact we are working with a copied data
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
