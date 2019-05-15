use std::env;
use config::{QueueConfig, ConnectionConfig};
use std::process::exit;

pub fn get_queue_configs() -> Vec<QueueConfig> {
    let mut queues: Vec<QueueConfig> = Vec::new();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        display_help();
        println!("Error: no queue names specified");
        exit(1);
    }

    for i in 1..args.len() {
        queues.push(QueueConfig {
            name: args[i].clone(),
        });
    }
    queues
}

pub fn get_connection_config<'a>() -> ConnectionConfig<'a> {
    let connection_config = ConnectionConfig {
        hostname: "command_queue_redis",
        port: 6379,
        timeout: 3,
    };
    connection_config
}

fn display_help() {
    println!("command-queue QUEUE_NAME [QUEUE_NAME...]");
}