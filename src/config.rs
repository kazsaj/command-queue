use std::fmt;

#[derive(Clone)]
pub struct ConnectionConfig {
    pub hostname: String,
    pub port: usize,
    pub pop_timeout: usize,
}

impl ConnectionConfig {
    pub fn get_connection_string(&self) -> String {
        let prefix = "redis://";

        let mut connection_string = String::new();
        connection_string.push_str(prefix);
        connection_string.push_str(&self.hostname);

        if self.port > 0 {
            let port_number = self.port.to_string();
            connection_string.push_str(":");
            connection_string.push_str(port_number.as_str());
        }

        connection_string
    }
}

impl fmt::Display for ConnectionConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConnectionConfig: {} with pop_timeout: {}", self.get_connection_string(), self.pop_timeout)
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

#[cfg(test)]
mod tests {
    use config::ConnectionConfig;
    use config::QueueConfig;

    #[test]
    fn get_connection_string() {
        let config = ConnectionConfig {
            hostname: "server_hostname".to_string(),
            port: 666,
            pop_timeout: 3,
        };
        assert_eq!(config.get_connection_string(), "redis://server_hostname:666");
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