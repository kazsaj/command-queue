extern crate redis;

use self::redis::Connection;
use crate::STOP;
use crate::config::{EnvConfig, ProcessConfig};
use crate::output::Logger;
use crate::worker::redis::Commands;
use std::process::Command;
use std::sync::atomic::Ordering;
use std::{thread, time};

#[derive(PartialEq)]
enum Status {
    FailedToPull,
    ExecutedCommand,
    FailedToExecute,
    ReceivedStopSignal,
}

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
    logger.debug(format!(
        "T#{} lists order: {:?}",
        thread_number,
        process_configs
            .iter()
            .map(|config| &config.pull_queue_name)
            .collect::<Vec<_>>(),
    ));

    loop {
        for i in 0..process_configs.len() {
            if check_stop(&thread_number, &logger) {
                return;
            }
            match pop_and_process(&thread_number, &logger, &env_config, &process_configs[i]) {
                Status::FailedToPull => continue, // continue will attempt to use a different queue
                Status::ExecutedCommand => break, // break will revert to the primary queue
                Status::FailedToExecute => break,
                Status::ReceivedStopSignal => return, // close the thread
            }
        }
    }
}

/// Check if received signal to stop and terminate
fn check_stop(thread_number: &usize, logger: &Logger) -> bool {
    if STOP.load(Ordering::Acquire) {
        logger.warning(format!("T#{} received STOP", thread_number));
        return true;
    }

    false
}

/// Pop a value from the specified queue and then try to process it
///
/// Returns true if queue had any value to process or if STOP was set
fn pop_and_process(
    thread_number: &usize,
    logger: &Logger,
    env_config: &EnvConfig,
    process_config: &ProcessConfig,
) -> Status {
    let mut redis_connection: Connection =
        match get_connection(&thread_number, &logger, &env_config) {
            Ok(connection) => connection,
            Err(_) => return Status::FailedToPull,
        };

    let pulled_value = pop_from_queue(
        &mut redis_connection,
        &env_config,
        &process_config.pull_queue_name,
    );
    if !pulled_value.is_ok() {
        // wasn't able to pull anything that we can process
        return Status::FailedToPull;
    }

    let raw_command = pulled_value.unwrap().1;

    logger.debug(format!(
        "T#{} pulled from {}: {}",
        thread_number, process_config.pull_queue_name, raw_command
    ));

    set_as_last_command(
        &logger,
        &mut redis_connection,
        &thread_number,
        &env_config,
        &raw_command,
    );

    let execute_result = execute_command(
        &thread_number,
        &logger,
        &env_config,
        &process_config,
        &raw_command,
    );

    if execute_result != Status::ExecutedCommand {
        push_to_queue(
            &logger,
            &mut redis_connection,
            &process_config.error_queue_name,
            raw_command.clone(),
        );
    }

    execute_result
}

/// Try to execute a command the specified number of times
fn execute_command(
    thread_number: &usize,
    logger: &Logger,
    env_config: &EnvConfig,
    process_config: &ProcessConfig,
    raw_command: &String,
) -> Status {
    for i in 1..env_config.retry_limit + 2 {
        let command_output = Command::new("sh")
            .arg("-c")
            .arg(raw_command.clone())
            .output()
            .expect("failed to execute process");

        if command_output.status.success() {
            logger.info(format!(
                "T#{} execute result for {} OK#{}/{}: {}",
                thread_number,
                process_config.pull_queue_name,
                i,
                env_config.retry_limit + 1,
                raw_command
            ));
            return Status::ExecutedCommand;
        }

        logger.warning(format!(
            "T#{} execute result for {} Err#{}/{}: {}",
            thread_number,
            process_config.pull_queue_name,
            i,
            env_config.retry_limit + 1,
            raw_command
        ));

        if check_stop(&thread_number, &logger) {
            return Status::ReceivedStopSignal;
        }

        if i != env_config.retry_limit + 1 {
            // only sleep if it's not the last attempt
            thread::sleep(time::Duration::from_secs(env_config.retry_sleep));
        }
    }

    logger.error(format!(
        "T#{} too many errors, adding to {}: {}",
        thread_number, process_config.error_queue_name, raw_command
    ));

    Status::FailedToExecute
}

/// Connect with redis
fn get_connection(
    thread_number: &usize,
    logger: &Logger,
    env_config: &EnvConfig,
) -> Result<Connection, String> {
    let connection_string = env_config.get_connection_string();
    let client = match redis::Client::open(connection_string.as_str()) {
        Ok(value) => value,
        Err(error) => {
            logger.error(format!(
                "T#{} could not connect to redis: {}",
                thread_number, error
            ));
            return Err(error.to_string());
        }
    };

    loop {
        if check_stop(&thread_number, &logger) {
            return Err("STOP".to_string());
        }

        match client.get_connection() {
            Ok(connection) => return Ok(connection),
            Err(error) => {
                logger.error(format!(
                    "T#{} could not connect to redis: {}",
                    thread_number, error
                ));
                thread::sleep(time::Duration::from_secs(30));
            }
        }
    }
}

/// Pop a value from a specified queue
fn pop_from_queue(
    redis_connection: &mut Connection,
    env_config: &EnvConfig,
    queue_name: &String,
) -> redis::RedisResult<(String, String)> {
    redis_connection.blpop(queue_name, env_config.redis_pop_timeout)
}

/// Add an entry to the end of a queue
fn push_to_queue(
    logger: &Logger,
    redis_connection: &mut Connection,
    queue_name: &String,
    data: String,
) {
    let push_result: redis::RedisResult<usize> = redis_connection.rpush(queue_name, data.clone());
    match push_result {
        Ok(_) => {}
        Err(error) => logger.error(format!(
            "Could not add \"{}\" to {}: {:?}",
            data, queue_name, error
        )),
    }
}

/// Report the last command we ran to redis
fn set_as_last_command(
    logger: &Logger,
    redis_connection: &mut Connection,
    thread_number: &usize,
    env_config: &EnvConfig,
    raw_command: &String,
) {
    let last_command_key = env_config.get_last_command_key(thread_number);
    let set_result: redis::RedisResult<()> = redis_connection.set_ex(
        last_command_key.clone(),
        raw_command,
        env_config.last_command_expire,
    );
    match set_result {
        Ok(_) => {}
        Err(error) => logger.error(format!(
            "Could not set \"{}\": {:?}",
            last_command_key, error
        )),
    }
}
