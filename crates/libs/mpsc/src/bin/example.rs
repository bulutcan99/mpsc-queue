use std::{
    sync::{atomic::AtomicUsize, Arc, Barrier},
    thread,
};

use mpsc::{channel, consumer, producer};

fn main() {
    let producer_count = Arc::new(AtomicUsize::new(0));
    let channel = Arc::new(channel::Channel::<String>::new(producer_count.clone()));

    let producer_handles: Vec<_> = (0..5)
        .map(|i| {
            let producer = producer::Producer::new(Arc::clone(&channel));
            thread::spawn(move || {
                let message = format!("Hello-Thread{}", i);
                producer.send(message);
            })
        })
        .collect();

    for handle in producer_handles {
        handle.join().expect("Producer thread panicked");
    }

    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = Arc::clone(&barrier);

    let consumer = consumer::Consumer::new(Arc::clone(&channel));
    consumer.consume(
        move |message| {
            println!("{}", message);
        },
        move || {
            barrier_clone.wait();
        },
    );

    println!("Consumer is done");
    barrier.wait();
}
