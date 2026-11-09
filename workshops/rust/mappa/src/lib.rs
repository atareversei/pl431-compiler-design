use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}

// We don't use #[derive(Clone)] on the struct itself because the generated implementation
// looks like `impl<T: Clone> Clone for Struct<T>`, but we don't want the T to be necessarily
// cloneable. We are cloning the Arc<Inner<T>>, not Inner<T>.
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            // We use Arc::clone because self.inner.clone() gets dereferenced to
            // clone the value inside of Arc, but instead we meant to clone the
            // Arc itself.
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct Reciever<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Reciever<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

pub struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Reciever<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Reciever {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}
