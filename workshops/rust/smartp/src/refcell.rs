use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    NotShared,
    Shared(isize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>, // The reason why we use Cell<T> is because we need to mutate the value
                           // without holding an exclusve reference in `fn borrow(&self)`
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::NotShared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::NotShared => {
                self.state.set(RefState::Shared(1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::NotShared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::NotShared | RefState::Exclusive => unreachable!(),
            RefState::Shared(n) => {
                if n == 1 {
                    self.refcell.state.set(RefState::NotShared);
                } else {
                    self.refcell.state.set(RefState::Shared(n - 1));
                };
            }
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::NotShared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::NotShared),
        }
    }
}
