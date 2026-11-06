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
        // SAFTEY: we know no one else is concurrently mutating self.value (because !Sync)
        // SAFTEY: we know we are not invalidating any references, because we never give any out
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
        // SAFTEY: we know no one else is modifting this value, since only this thread
        // can mutate (because !Sync).
        unsafe { *self.value.get() }
    }
}
