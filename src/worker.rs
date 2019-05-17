extern crate redis;

use worker::redis::Commands;
use config::{ConnectionConfig, QueueConfig};
use output;
use std::{thread, time};

pub fn main(thread_number: usize, config: ConnectionConfig, queue: QueueConfig, other_queues: Vec<QueueConfig>) {
    output::info(format!("thread #{} using {}", thread_number, queue));
    loop {
        for i in 0..other_queues.len() {
            // first try to process the main queue
            if pop_and_process(thread_number, &config, &queue, true) {
                break;
            }
            if pop_and_process(thread_number, &config, &queue, false) {
                break;
            }
            // nothing to process, use fall back queue
            if pop_and_process(thread_number, &config, &other_queues[i], true) {
                break;
            }
            if pop_and_process(thread_number, &config, &other_queues[i], false) {
                break;
            }
        }
    }
}

/// Pop a value from the specified queue and then try to process it
///
/// Returns true if queue had any value to process
fn pop_and_process(thread_number: usize, config: &ConnectionConfig, queue: &QueueConfig, priority: bool) -> bool {
    let queue_name = match priority {
        true => queue.get_priority_queue_name(),
        false => queue.get_default_queue_name(),
    };

    let pulled_value = pop_from_queue(&config, &queue_name);
    let pull_result = pulled_value.is_ok();

    match pulled_value {
        Ok(value) => output::info(format!("thread #{} pulled from {}: {}", thread_number, queue_name, value.1)),
        Err(_) => { /* do nothing, queues can be empty sometimes */ },
    }

    pull_result
}

/// Pop a value from a specified queue
fn pop_from_queue(config: &ConnectionConfig, queue_name: &String) -> redis::RedisResult<(String, isize)> {
    let connection_string = config.get_connection_string();
    let client = match redis::Client::open(connection_string.as_str()) {
        Ok(client) => client,
        Err(error) => {
            output::error(format!("Could not connect to redis: {:?}", error));
            thread::sleep(time::Duration::from_secs(180));
            return Err(error);
        },
    };
    let connection = match client.get_connection() {
        Ok(connection) => connection,
        Err(error) => {
            output::warning(format!("Could not connect to redis: {:?}", error));
            thread::sleep(time::Duration::from_secs(60));
            return Err(error);
        },
    };
    connection.blpop(queue_name, config.pop_timeout)
}