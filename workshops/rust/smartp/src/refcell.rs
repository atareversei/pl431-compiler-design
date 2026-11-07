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
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::NotShared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
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
impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // A Ref is only created if no exclusive references have been given out. Once it is
        // given out, state is set to Shared, so no exclusive references are given out.
        // As a result, dereferencing into a shared reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::NotShared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::NotShared),
        }
    }
}
impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // See SAFETY for DerefMut.
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // A RefMut is only created if no other references have been given out. Once it is
        // given out, state is set to Exclusive, so no future references are given out.
        // As a result, we have an exclusive lease on the inner value, so mutably
        // dereferencing is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}
