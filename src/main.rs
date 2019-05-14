mod config;
mod worker;

use std::thread;

fn main() {
    println!("Starting main thread");

    let configured_threads_count = 2;
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    let queue_name = "queue";

    for i in 0..configured_threads_count {
        println!("Spawning worker thread {} using list {}", i, queue_name);
        let thread_number = i.clone();
        threads.push(thread::spawn(move || worker::main(thread_number, queue_name)));
    }

    // wait for all the threads to finish before exiting
    for j in 0..threads.len() {
        match threads.pop() {
            Some(i) => println!("Thread {} joined {:?}", j, i.join()),
            None => println!("Could not pop {} thread from vector", j),
        }
    }

}
