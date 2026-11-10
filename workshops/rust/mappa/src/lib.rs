use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Flavors:
// - Synchronous channels: Channel where send() can block. Limited capacity.
//      - Mutex + Convar + VecDequ
//      - Atomic VecDeque: (atomic queue) + thread::park + thread::Thread::notify
// - Asynchronous channels: Channel where send() cannot block. Unbounded.
//      - Mutex + Convar + VecDequ
//      - Mutex + Convar + LinkedList
//      - Atomic LinkedList, linked list of T
//      - Atomic Block LinkedList, linked list of VecDeque<T>
// - Rendevouz channels: Synchronous with capacity = 0. Used for thread synchronization.
// - Oneshot channels: Any capacity. In practice, only one call to send ().

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

// We don't use #[derive(Clone)] on the struct itself because the generated implementation
// looks like `impl<T: Clone> Clone for Struct<T>`, but we don't want the T to be necessarily
// cloneable. We are cloning the Arc<Shared<T>>, not Inner<T>.
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Self {
            // We use Arc::clone because self.shared.clone() gets dereferenced to
            // clone the value inside of Arc, but instead we meant to clone the
            // Arc itself.
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

pub struct Reciever<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Reciever<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if Arc::strong_count(&self.shared) == 0 => {
                    return None;
                }
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

pub struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Reciever<T>) {
    let shared = Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            senders: 0,
        }),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Reciever {
            shared: shared.clone(),
            buffer: VecDeque::new(),
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
        assert_eq!(rx.recv(), Some(42));
    }
}
