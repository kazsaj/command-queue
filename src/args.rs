use std::env;
use config::{QueueConfig, ConnectionConfig};
use std::process::exit;
use output;

pub fn get_queue_configs() -> Vec<QueueConfig> {
    let mut queues: Vec<QueueConfig> = Vec::new();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        output::error(format!("{}", "No queue names specified, see --help"));
        exit(1)
    }

    for i in 1..args.len() {
        if args[i].eq("--help") {
            display_help();
            exit(0);
        }

        queues.push(QueueConfig {
            name: args[i].clone(),
        });
    }
    queues
}

pub fn get_connection_config() -> ConnectionConfig {
    let connection_config = ConnectionConfig {
        hostname: "127.0.0.1".to_string(),
        port: 6379,
        timeout: 3,
    };
    connection_config
}

fn display_help() {
    println!("command-queue QUEUE_NAME [QUEUE_NAME...]");
}