use crate::channel::Channel;
use std::{sync::Arc, thread};

pub struct Consumer<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Consumer<T> {
    pub fn new(channel: Arc<Channel<T>>) -> Self {
        Self { channel }
    }

    pub fn consume<F, G>(&self, mut process: F, done: G)
    where
        F: FnMut(T) + Send + 'static,
        G: FnOnce() + Send + 'static,
        T: Send + 'static,
    {
        let channel = Arc::clone(&self.channel);
        thread::spawn(move || {
            while let Some(message) = channel.recv() {
                process(message);
            }
            done();
        });
    }
}
