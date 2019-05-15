use std::fmt;

#[derive(Clone, Debug)]
pub struct ConnectionConfig<'a> {
    pub hostname: &'a str,
    pub port: usize,
    pub timeout: usize,
}

impl ConnectionConfig<'_> {
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

        return connection_string;
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
        return queue_name;
    }
    pub fn get_default_queue_name(&self) -> String {
        let mut queue_name = self.name.clone();
        queue_name.push_str("_default");
        return queue_name;
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
            hostname: "server_hostname",
            port: 666,
            timeout: 3,
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
    }
}