use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

/// Channel has 3 parts:
/// 1. A queue to store the messages
/// 2. A condition variable to notify the receiver that there are messages available.
/// 3. A producer count to keep track of the number of producers.
#[derive(Debug, Default)]
pub struct Channel<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    pub available: Condvar,
    pub producer_count: Arc<AtomicUsize>,
}

impl<T> Channel<T> {
    pub fn new(producer_count: Arc<AtomicUsize>) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            available: Condvar::new(),
            producer_count,
        }
    }

    pub fn send(&self, message: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(message);
        self.available.notify_one();
    }

    pub fn recv(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            if self.producer_count.load(Ordering::SeqCst) == 0 {
                return None;
            }
            queue = self.available.wait(queue).unwrap();
        }
        queue.pop_front()
    }
}
