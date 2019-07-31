extern crate redis;

use config::{EnvConfig, ProcessConfig};
use output::Logger;
use std::process::Command;
use std::sync::atomic::Ordering;
use std::{thread, time};
use worker::redis::Commands;
use STOP;

pub fn main(
    thread_number: usize,
    logger: Logger,
    env_config: EnvConfig,
    process_configs: Vec<ProcessConfig>,
) {
    logger.info(format!(
        "T#{} spawned checking {} lists",
        thread_number,
        process_configs.len(),
    ));

    loop {
        for i in 0..process_configs.len() {
            if STOP.load(Ordering::Acquire) {
                return;
            }
            if pop_and_process(&thread_number, &logger, &env_config, &process_configs[i]) {
                break;
            }
        }
    }
}

/// Pop a value from the specified queue and then try to process it
///
/// Returns true if queue had any value to process or if STOP was set
fn pop_and_process(
    thread_number: &usize,
    logger: &Logger,
    env_config: &EnvConfig,
    process_config: &ProcessConfig,
) -> bool {
    let pulled_value = pop_from_queue(&env_config, &logger, &process_config.pull_queue_name);
    if !pulled_value.is_ok() {
        // wasn't able to pull anything that we can process
        return false;
    }

    let raw_command = pulled_value.unwrap().1;

    logger.debug(format!(
        "T#{} pulled from {}: {}",
        thread_number, process_config.pull_queue_name, raw_command
    ));

    for i in 1..env_config.retry_limit + 2 {
        let command_output = Command::new("sh")
            .arg("-c")
            .arg(raw_command.clone())
            .output()
            .expect("failed to execute process");

        if command_output.status.success() {
            logger.info(format!(
                "T#{} execute result for {} OK#{}/{}: {}",
                thread_number, process_config.pull_queue_name, i, env_config.retry_limit + 1, raw_command
            ));
            return true;
        }

        logger.warning(format!(
            "T#{} execute result for {} Err#{}/{}: {}",
            thread_number, process_config.pull_queue_name, i, env_config.retry_limit + 1, raw_command
        ));

        // sigterm received, better gracefully exit than retry
        if STOP.load(Ordering::Acquire) {
            break;
        }

        if i != env_config.retry_limit {
            // only sleep if it's not the last attempt
            thread::sleep(time::Duration::from_secs(env_config.retry_sleep));
        }
    }

    logger.error(format!(
        "T#{} too many errors, adding to {}: {}",
        thread_number, process_config.error_queue_name, raw_command
    ));
    match push_to_queue(
        env_config,
        &process_config.error_queue_name,
        raw_command.clone(),
    ) {
        Ok(_) => {}
        Err(error) => logger.error(format!(
            "Could not add \"{}\" to {}: {:?}",
            raw_command, process_config.error_queue_name, error
        )),
    };

    true
}

/// Pop a value from a specified queue
fn pop_from_queue(
    env_config: &EnvConfig,
    logger: &Logger,
    queue_name: &String,
) -> redis::RedisResult<(String, String)> {
    let connection_string = env_config.get_connection_string();
    let client = match redis::Client::open(connection_string.as_str()) {
        Ok(client) => client,
        Err(error) => {
            logger.error(format!("Could not connect to redis: {:?}", error));
            thread::sleep(time::Duration::from_secs(180));
            return Err(error);
        }
    };
    let connection = match client.get_connection() {
        Ok(connection) => connection,
        Err(error) => {
            logger.warning(format!("Could not connect to redis: {:?}", error));
            thread::sleep(time::Duration::from_secs(60));
            return Err(error);
        }
    };
    connection.blpop(queue_name, env_config.redis_pop_timeout)
}

fn push_to_queue(
    env_config: &EnvConfig,
    queue_name: &String,
    data: String,
) -> redis::RedisResult<usize> {
    let connection_string = env_config.get_connection_string();
    let client = match redis::Client::open(connection_string.as_str()) {
        Ok(client) => client,
        Err(error) => {
            return Err(error);
        }
    };
    let connection = match client.get_connection() {
        Ok(connection) => connection,
        Err(error) => {
            return Err(error);
        }
    };
    connection.rpush(queue_name, data)
}
