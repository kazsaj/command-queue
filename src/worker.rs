extern crate redis;

use worker::redis::Commands;
use config::{ConnectionConfig, QueueConfig};
use ::{output, STOP};
use std::{thread, time};
use std::sync::atomic::Ordering;
use std::process::Command;

pub fn main(thread_number: usize, config: ConnectionConfig, queue: QueueConfig, other_queues: Vec<QueueConfig>) {
    output::info(format!("thread #{} using {}", thread_number, queue));
    while !STOP.load(Ordering::Acquire) {
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
/// Returns true if queue had any value to process or if STOP was set
fn pop_and_process(thread_number: usize, config: &ConnectionConfig, queue: &QueueConfig, priority: bool) -> bool {
    if STOP.load(Ordering::Acquire) {
        return true;
    }

    let queue_name = match priority {
        true => queue.get_priority_queue_name(),
        false => queue.get_default_queue_name(),
    };

    let pulled_value = pop_from_queue(&config, &queue_name);
    if !pulled_value.is_ok() {
        // wasn't able to pull anything that we can process
        return false;
    }

    let raw_command = pulled_value.unwrap().1;
    let sleep: u64 = 31;
    let attempts: usize = 3;

    for i in 0..attempts {
        let command_output = Command::new("sh")
            .arg("-c")
            .arg(raw_command.clone())
            .output()
            .expect("failed to execute process");

        if command_output.status.success() {
            output::info(format!("thread #{} pulled from {} OK#{}: {}", thread_number, queue_name, i, raw_command));
            return true;
        }

        output::warning(format!("thread #{} pulled from {} Err#{}: {}", thread_number, queue_name, i, raw_command));

        // sigterm received, better gracefully exit than retry
        if STOP.load(Ordering::Acquire) {
            break;
        }

        thread::sleep(time::Duration::from_secs(sleep));
    }

    // todo move to error list here

    true
}

/// Pop a value from a specified queue
fn pop_from_queue(config: &ConnectionConfig, queue_name: &String) -> redis::RedisResult<(String, String)> {
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