use ::mpsc::{channel, consumer, producer};
use std::{sync::Arc, thread};

fn main() {
    let channel = Arc::new(channel::Channel::<Option<String>>::new());
    let producer = Arc::new(producer::Producer::new(Arc::clone(&channel)));

    // Producer threads
    let producer_handles: Vec<_> = (0..5)
        .map(|i| {
            let producer = Arc::clone(&producer);
            thread::spawn(move || {
                let message = format!("Hello-Thread{}", i);
                producer.send(Some(message)); // Send a message
            })
        })
        .collect();

    // Wait for all producer threads to complete
    for handle in producer_handles {
        handle.join().expect("Producer thread panicked");
    }

    // Send `None` to indicate all messages are sent
    producer.send(None);

    // Start the consumer in a separate thread
    let consumer = consumer::Consumer::new(Arc::clone(&channel));
    let consumer_handle = thread::spawn(move || {
        consumer.consume(|message| {
            if let Some(msg) = message {
                println!("{}", msg);
            } else {
                // Exit if a None message is received
                return false;
            }
            true
        });
    });

    // Wait for the consumer thread to finish
    consumer_handle.join().expect("Consumer thread panicked");
}
