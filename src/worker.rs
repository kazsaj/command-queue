extern crate redis;

use std::thread;
use std::time::Duration;
use worker::redis::Commands;

pub fn main(thread_number: i32, queue_name: &str)
{
    for i in 1..10 {
        thread::sleep(Duration::from_millis(1));
        let pulled_value = pop_from_queue(queue_name);
        match pulled_value {
            Ok(value) => println!("{} pull {}/{}: {}", queue_name, thread_number, i, value.1),
            Err(value) => println!("{} pull {}/{}: {}", queue_name, thread_number, i, value),
        }
    }
}

fn pop_from_queue(queue_name: &str) -> redis::RedisResult<(String, isize)> {
    let client = redis::Client::open("redis://command_queue_redis/")?;
    let connection = client.get_connection()?;
    connection.blpop(queue_name, 10)
}