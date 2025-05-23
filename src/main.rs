extern crate graceful;
extern crate rand;

mod args;
mod config;
mod output;
mod worker;

use config::{Priority, ProcessConfig, QueueConfig};
use graceful::SignalGuard;
use rand::rng;
use rand::seq::SliceRandom;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static STOP: AtomicBool = AtomicBool::new(false);

fn main() {
    let logger = args::get_logger();
    let queues = args::get_queue_configs(&logger);
    let env_config = args::get_env_config();
    let signal_guard = SignalGuard::new();

    logger.info(format!(
        "Spawning {} threads using {}, {}",
        queues.len(),
        env_config,
        logger,
    ));

    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    for i in 0..queues.len() {
        let thread_logger = logger.clone();
        let thread_queue = queues[i].clone();
        let thread_config = env_config.clone();
        let thread_process_configs = get_process_configs(&queues, thread_queue);

        threads.push(thread::spawn(move || {
            worker::main(i + 1, thread_logger, thread_config, thread_process_configs)
        }));
    }

    signal_guard.at_exit(move |sig| {
        logger.warning(format!("Signal {} received.", sig));
        STOP.store(true, Ordering::Release);

        // wait for all the threads to finish before exiting
        for i in 1..threads.len() + 1 {
            match threads.pop() {
                Some(thread) => {
                    let _ = thread.join();
                    logger.info(format!("T#{} finished", i))
                }
                None => logger.error(format!("T#{} failed to join", i)),
            }
        }

        logger.info(format!("All threads finished"));
    });
}

fn get_process_configs(queues: &Vec<QueueConfig>, thread_queue: QueueConfig) -> Vec<ProcessConfig> {
    let mut process_configs: Vec<ProcessConfig> = Vec::new();
    process_configs.push(ProcessConfig::new(&thread_queue, Priority::High));
    process_configs.push(ProcessConfig::new(&thread_queue, Priority::Default));
    process_configs.push(ProcessConfig::new(&thread_queue, Priority::Low));

    // remove instance of the thread queue from the list, to avoid trying to process it twice
    let other_queues = get_remaining_queues(queues, &thread_queue);
    for i in 0..other_queues.len() {
        process_configs.push(ProcessConfig::new(&other_queues[i], Priority::High));
        process_configs.push(ProcessConfig::new(&other_queues[i], Priority::Default));
        process_configs.push(ProcessConfig::new(&other_queues[i], Priority::Low));
    }

    process_configs
}

/// Returns a vector including all QueueConfigs except the one specified as the second param
fn get_remaining_queues(queues: &Vec<QueueConfig>, exclude: &QueueConfig) -> Vec<QueueConfig> {
    let mut other_queues: Vec<QueueConfig> = Vec::new();

    for k in 0..queues.len() {
        if queues[k].name == exclude.name {
            continue;
        }
        if other_queues.contains(&queues[k]) {
            continue;
        }
        let copied_config = queues[k].clone();
        other_queues.push(copied_config);
    }

    other_queues.shuffle(&mut rng());
    other_queues
}
