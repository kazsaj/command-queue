extern crate rand;

mod args;
mod config;
mod worker;

use std::thread;
use config::QueueConfig;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {

    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    let queues = args::get_queue_configs();
    let connection_config = args::get_connection_config();

    println!("Spawning {} threads", queues.len());

    for i in 0..queues.len() {
        let thread_queue = queues[i].clone();
        let thread_number = i.clone();
        let thread_config = connection_config.clone();
        let other_queues = get_remaining_queues(&queues, &thread_queue);
        threads.push(thread::spawn(move || worker::main(thread_number, thread_config, thread_queue, other_queues)));
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

fn get_remaining_queues(queues: &Vec<QueueConfig>, exclude: &QueueConfig) -> Vec<QueueConfig> {
    let mut other_queues: Vec<QueueConfig> = Vec::new();

    // remove from cloned queue list, to avoid duplicates
    for k in 0..queues.len() {
        if queues[k].name == exclude.name {
            continue;
        }
        let copied_config = queues[k].clone();
        other_queues.push(copied_config);
    }

    other_queues.shuffle(&mut thread_rng());
    other_queues
}