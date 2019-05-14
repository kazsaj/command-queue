extern crate redis;

use worker::redis::Commands;
use config::ConnectionConfig;

pub fn main(thread_number: i32, config: ConnectionConfig, queue_name: &str)
{
    for i in 1..10 {
        let pulled_value = pop_from_queue(&config, queue_name);
        match pulled_value {
            Ok(value) => println!("{} pull {}/{}: {}", queue_name, thread_number, i, value.1),
            Err(value) => println!("{} pull {}/{}: {}", queue_name, thread_number, i, value),
        }
    }
}

fn pop_from_queue(config: &ConnectionConfig, queue_name: &str) -> redis::RedisResult<(String, isize)> {
    let connection_string = config.get_connection_string();
    let client = redis::Client::open(connection_string.as_str())?;
    let connection = client.get_connection()?;
    connection.blpop(queue_name, config.timeout)
}