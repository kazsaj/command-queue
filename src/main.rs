mod config;
mod worker;

use std::thread;

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
        match threads.pop() {
            Some(i) => println!("Thread {} joined {:?}", j, i.join()),
            None => println!("Could not pop {} thread from vector", j),
        }
    }

}
