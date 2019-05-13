extern crate redis;

use std::thread;
use std::time::Duration;
use worker::redis::Commands;

pub fn main()
{
    let queue_name = "queue";
    for i in 1..10 {
        println!("worker pull number {}", i);
        thread::sleep(Duration::from_millis(1));
        let pulled_value = pop_from_queue(queue_name);
        match pulled_value {
            Ok(value) => println!("got a value: {}", value.1),
            Err(value) => println!("an error occurred: {}", value),
        }
    }
}

fn pop_from_queue(queue_name: &str) -> redis::RedisResult<(String, isize)> {
    let client = redis::Client::open("redis://command_queue_redis/")?;
    let connection = client.get_connection()?;
    connection.blpop(queue_name, 10)
}