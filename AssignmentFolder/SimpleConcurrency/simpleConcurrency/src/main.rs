/*
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}, started", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads completed.");
}
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handle = thread::spawn({
        let counter_clone = Arc::clone(&counter);
        move|| {
            for i in 1..=5 {
                println!("Thread {}, started", i);
                for j in 0..10{
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
            }
        }
    });

    handle.join().unwrap();
    let value = *counter.lock().unwrap();
    println!("Final counter value: {}", value);
}
