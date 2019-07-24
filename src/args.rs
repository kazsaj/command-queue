use config::{EnvConfig, QueueConfig};
use output::{LogLevel, Logger};
use std::env;
use std::process::exit;

/// Generate a logger based on the environment variable
pub fn get_logger() -> Logger {
    Logger {
        log_level: match env::var("COMMAND_QUEUE_LOG_LEVEL") {
            Ok(value) => LogLevel::from_string(value),
            Err(_) => LogLevel::get_default(),
        },
    }
}

/// Returns a vector with all the QueueConfigs that have been passed as arguments
///
/// Will exit if no queues have been specified.
pub fn get_queue_configs(logger: &Logger) -> Vec<QueueConfig> {
    let mut queues: Vec<QueueConfig> = Vec::new();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        logger.error(format!("{}", "No queue names specified, see --help"));
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

/// Returns connection configuration to Redis
pub fn get_env_config() -> EnvConfig {
    let hostname: String = match env::var("COMMAND_QUEUE_REDIS_HOSTNAME") {
        Ok(value) => value,
        Err(_) => "127.0.0.1".to_string(),
    };
    let port: usize = match env::var("COMMAND_QUEUE_REDIS_PORT") {
        Ok(value) => value.parse::<usize>().unwrap(),
        Err(_) => 6379,
    };
    let pop_timeout: usize = match env::var("COMMAND_QUEUE_REDIS_POP_TIMEOUT") {
        Ok(value) => value.parse::<usize>().unwrap(),
        Err(_) => 3,
    };
    let retry_sleep: u64 = match env::var("COMMAND_QUEUE_RETRY_SLEEP") {
        Ok(value) => value.parse::<u64>().unwrap(),
        Err(_) => 31,
    };
    let retry_limit: usize = match env::var("COMMAND_QUEUE_RETRY_LIMIT") {
        Ok(value) => value.parse::<usize>().unwrap(),
        Err(_) => 2,
    };

    let env_config = EnvConfig {
        hostname,
        port,
        pop_timeout,
        retry_sleep,
        retry_limit,
    };
    env_config
}

/// Display generic help message
fn display_help() {
    println!("command-queue QUEUE_NAME [QUEUE_NAME...]");
}
