mod config;
mod worker;

use std::thread;
use config::ConnectionConfig;
use config::QueueConfig;

fn main() {
    println!("Starting main thread");

    let configured_threads_count: usize = 4;
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    let main_queue = QueueConfig {
        name: "queue"
    };
    let connection_config = ConnectionConfig {
        hostname: "command_queue_redis",
        port: 6379,
        timeout: 3,
    };

    for i in 0..configured_threads_count {
        println!("Spawning worker thread {} using list {:#?}", i, main_queue);
        let thread_number = i.clone();
        let thread_config = connection_config.clone();
        let thread_queue = main_queue.clone();
        threads.push(thread::spawn(move || worker::main(thread_number, thread_config, thread_queue)));
    }

    // wait for all the threads to finish before exiting
    for j in 0..threads.len() {
        match threads.pop() {
            Some(i) => println!("Thread {} joined {:?}", j, i.join()),
            None => println!("Could not pop {} thread from vector", j),
        }
    }

    println!("All threads finished");
}
