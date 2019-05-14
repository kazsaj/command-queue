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

#[cfg(test)]
mod tests {
    use config::ConnectionConfig;

    #[test]
    fn get_connection_string() {
        let config = ConnectionConfig {
            hostname: "server_hostname",
            port: 666,
            timeout: 3,
        };
        assert_eq!(config.get_connection_string(), "redis://server_hostname:666");
    }
}