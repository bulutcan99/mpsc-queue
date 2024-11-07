use std::sync::Arc;

use crate::channel::Channel;

pub struct Producer<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Producer<T> {
    pub fn new(channel: Arc<Channel<T>>) -> Self {
        Self { channel }
    }
    pub fn send(&self, message: T) {
        self.channel.send(message);
    }
}

impl<T> Drop for Producer<T> {
    fn drop(&mut self) {
        self.channel
            .producer_count
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        self.channel.available.notify_all();
    }
}
