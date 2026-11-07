use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}
impl<T> Rc<T> {
    fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            // SAFETY
            // Box does not return a null pointer as it returns a heap allocation.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner: &RcInner<T> = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}
impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // `self.inner` is a box that is only deallocated when the last Rc goes away.
        // We have an Rc, therefore the Box has not been deallocated, thus the deref is valid.
        &unsafe { self.inner.as_ref() as &RcInner<T> }.value
    }
}
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner: &RcInner<T> = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}
