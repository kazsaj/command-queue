extern crate redis;

use worker::redis::Commands;
use config::ConnectionConfig;
use config::QueueConfig;

pub fn main(thread_number: usize, config: ConnectionConfig, queue: QueueConfig) {
    println!("Spawning worker thread {} using list {:#?}", thread_number, queue);
    for _i in 1..10 {
        if pop_and_process(thread_number, &config, &queue, true) {
            continue;
        }
        pop_and_process(thread_number, &config, &queue, false);
    }
}

fn pop_and_process(thread_number: usize, config: &ConnectionConfig, queue: &QueueConfig, priority: bool) -> bool {
    let queue_name = match priority {
        true => queue.get_priority_queue_name(),
        false => queue.get_default_queue_name(),
    };

    let pulled_value = pop_from_queue(&config, &queue_name);
    let pull_result = pulled_value.is_ok();

    match pulled_value {
        Ok(value) => println!("{} pull {}: {}", queue_name, thread_number, value.1),
        Err(value) => println!("{} pull {}: {}", queue_name, thread_number, value),
    }

    return pull_result;
}

fn pop_from_queue(config: &ConnectionConfig, queue_name: &String) -> redis::RedisResult<(String, isize)> {
    let connection_string = config.get_connection_string();
    let client = redis::Client::open(connection_string.as_str())?;
    let connection = client.get_connection()?;
    connection.blpop(queue_name, config.timeout)
}