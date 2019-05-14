use std::env;
use config::{QueueConfig, ConnectionConfig};

pub fn get_queue_configs<'a>() -> Vec<QueueConfig<'a>> {
    let mut queues: Vec<QueueConfig> = Vec::new();
    queues.push(QueueConfig {
        name: "alpha",
    });
    queues.push(QueueConfig {
        name: "bravo",
    });
    queues.push(QueueConfig {
        name: "charlie",
    });
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