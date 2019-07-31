use std::fmt;

#[derive(Clone)]
pub struct EnvConfig {
    pub instance_name: String,
    pub redis_hostname: String,
    pub redis_port: usize,
    pub redis_pop_timeout: usize,
    pub retry_sleep: u64,
    pub retry_limit: usize,
}

impl EnvConfig {
    pub fn get_connection_string(&self) -> String {
        let connection_string: String = format!("redis://{}:{}", self.redis_hostname, self.redis_port);
        connection_string
    }
}

impl fmt::Display for EnvConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "instance_name: {}, redis_hostname: {}, redis_pop_timeout: {}, retry_limit: {}, retry_sleep: {}",
            self.instance_name,
            self.get_connection_string(),
            self.redis_pop_timeout,
            self.retry_limit,
            self.retry_sleep,
        )
    }
}

#[derive(Clone)]
pub struct QueueConfig {
    pub name: String,
}

impl QueueConfig {
    pub fn get_priority_queue_name(&self) -> String {
        let mut queue_name = self.name.clone();
        queue_name.push_str("_priority");
        queue_name
    }
    pub fn get_default_queue_name(&self) -> String {
        let mut queue_name = self.name.clone();
        queue_name.push_str("_default");
        queue_name
    }
    pub fn get_error_queue_name(&self) -> String {
        let mut queue_name = self.name.clone();
        queue_name.push_str("_error");
        queue_name
    }
}

impl fmt::Display for QueueConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QueueConfig: {}", self.name)
    }
}

pub struct ProcessConfig {
    pub pull_queue_name: String,
    pub error_queue_name: String,
}

impl ProcessConfig {
    pub fn new(queue_config: &QueueConfig, priority: bool) -> ProcessConfig {
        ProcessConfig {
            pull_queue_name: if priority {
                queue_config.get_priority_queue_name()
            } else {
                queue_config.get_default_queue_name()
            },
            error_queue_name: queue_config.get_error_queue_name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use config::EnvConfig;
    use config::QueueConfig;

    #[test]
    fn get_connection_string() {
        let config = EnvConfig {
            instance_name: "some-name".to_string(),
            redis_hostname: "server_hostname".to_string(),
            redis_port: 666,
            redis_pop_timeout: 3,
            retry_sleep: 31,
            retry_limit: 3,
        };
        assert_eq!(
            config.get_connection_string(),
            "redis://server_hostname:666"
        );
    }

    #[test]
    fn get_queue_name() {
        let queue = QueueConfig {
            name: "hello".to_string(),
        };
        assert_eq!(queue.get_priority_queue_name(), "hello_priority");
        assert_eq!(queue.get_default_queue_name(), "hello_default");
        assert_eq!(queue.get_error_queue_name(), "hello_error");
    }
}
