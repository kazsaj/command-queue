mod config;
mod worker;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting main thread");

    let configured_threads_count = 2;
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

    for i in 0..configured_threads_count {
        println!("Spawning worker thread {}", i);
        threads.push(thread::spawn(|| worker::main()));
    }

    // more main thread logic here

    for j in 0..threads.len() {
        println!("joining worker {}", j);
        let mut popped_thread = threads.pop();
        popped_thread.unwrap().join();
    }

}
