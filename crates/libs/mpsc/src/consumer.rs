use crate::channel::Channel;
use std::{sync::Arc, thread};

pub struct Consumer<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Consumer<T> {
    pub fn new(channel: Arc<Channel<T>>) -> Self {
        Self { channel }
    }

    pub fn consume<F>(&self, mut process: F)
    where
        F: FnMut(T) + Send + 'static,
        T: Send + 'static,
    {
        let channel = Arc::clone(&self.channel); // Kanalın kopyasını oluştur
        thread::spawn(move || loop {
            let message = channel.recv();
            process(message);
        });
    }
}
