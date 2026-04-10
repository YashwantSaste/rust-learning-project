//! Multi-threading and channels.
//!
//! The examples in this module cover thread spawning, moving owned values into
//! threads, and sending messages across channels.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Runs the concurrency examples.
pub fn run() {
    println!("== Concurrency ==");

    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("This is a thread! {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 0..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let my_string = String::from("Hello");
    let handle = thread::spawn(move || {
        let mut message = my_string;
        for _ in 0..3 {
            message.push_str(" World");
            thread::sleep(Duration::from_millis(1));
        }
        println!("This is a thread with move keyword! {}", message);
    });
    handle.join().unwrap();

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        sender.send(String::from("hi")).unwrap();
    });
    let received_message = receiver.recv().unwrap();
    println!("Received message: {}", received_message);

    let (sender1, common_receiver) = mpsc::channel();
    let sender2 = sender1.clone();

    thread::spawn(move || {
        let values1 = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("first"),
            String::from("thread"),
        ];

        for value in values1 {
            sender1.send(value).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::spawn(move || {
        let values2 = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for value in values2 {
            sender2.send(value).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    for received_message in common_receiver {
        println!("Received: {}", received_message);
    }
}
