extern crate graceful;
extern crate rand;

mod args;
mod config;
mod output;
mod worker;

use config::QueueConfig;
use graceful::SignalGuard;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static STOP: AtomicBool = AtomicBool::new(false);

fn main() {
    let queues = args::get_queue_configs();
    let env_config = args::get_env_config();
    let signal_guard = SignalGuard::new();

    output::info(format!(
        "Spawning {} threads using {}",
        queues.len(),
        env_config
    ));

    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    for i in 0..queues.len() {
        let thread_queue = queues[i].clone();
        let thread_number = i.clone();
        let thread_config = env_config.clone();
        // remove instance of the thread queue from the list, to avoid trying to process it twice
        let other_queues = get_remaining_queues(&queues, &thread_queue);
        threads.push(thread::spawn(move || {
            worker::main(thread_number, thread_config, thread_queue, other_queues)
        }));
    }

    signal_guard.at_exit(move |sig| {
        output::warning(format!("Signal {} received.", sig));
        STOP.store(true, Ordering::Release);

        // wait for all the threads to finish before exiting
        for i in 0..threads.len() {
            match threads.pop() {
                Some(_) => output::info(format!("T#{} finished", i)),
                None => output::error(format!("T#{} failed to join", i)),
            }
        }

        output::info(format!("All threads finished"));
    });
}

/// Returns a vector including all QueueConfigs except the one specified as the second param
fn get_remaining_queues(queues: &Vec<QueueConfig>, exclude: &QueueConfig) -> Vec<QueueConfig> {
    let mut other_queues: Vec<QueueConfig> = Vec::new();

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
