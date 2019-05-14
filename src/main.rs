extern crate rand;

mod config;
mod worker;

use std::thread;
use config::ConnectionConfig;
use config::QueueConfig;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Starting main thread");

    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    let mut queues: Vec<QueueConfig> = Vec::new();
    queues.push(QueueConfig {
        name: "alpha",
    });
    queues.push(QueueConfig {
        name: "bravo",
    });
    queues.push(QueueConfig {
        name: "charlie",
    });

    let connection_config = ConnectionConfig {
        hostname: "command_queue_redis",
        port: 6379,
        timeout: 3,
    };

    for i in 0..queues.len() {
        let thread_queue = queues[i].clone();
        let thread_number = i.clone();
        let thread_config = connection_config.clone();
        let mut queues_copy: Vec<QueueConfig> = Vec::new();

        // remove from cloned queue list, to avoid duplicates
        for k in 0..queues.len() {
            if queues[k].name == thread_queue.name {
                continue;
            }
            let copied_config = queues[k].clone();
            queues_copy.push(copied_config);
        }

        queues_copy.shuffle(&mut thread_rng());

        threads.push(thread::spawn(move || worker::main(thread_number, thread_config, thread_queue, queues_copy)));
    }

    // wait for all the threads to finish before exiting
    for j in 0..threads.len() {
        match threads.pop() {
            Some(thread) => println!("Thread {} joined {:?}", j, thread.join()),
            None => println!("Could not pop {} thread from vector", j),
        }
    }

    println!("All threads finished");
}
