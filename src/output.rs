use std::time::SystemTime;
use std::fmt;

#[derive(Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl LogLevel {
    /// Convert the LogLevel enum to a numeric representation making it easier to compare them
    fn to_number(&self) -> usize {
        return match self {
            LogLevel::Error => 3,
            LogLevel::Warning => 2,
            LogLevel::Info => 1,
            LogLevel::Debug => 0,
        };
    }

    /// Convert the LogLevel to a output suitable str
    fn to_str(&self) -> &str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }

    /// Return the LogLevel based on a string representation
    pub fn from_string(value: String) -> LogLevel {
        match value.to_ascii_uppercase().as_str() {
            "ERROR" => LogLevel::Error,
            "WARNING" => LogLevel::Warning,
            "WARN" => LogLevel::Warning,
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            _ => LogLevel::get_default(),
        }
    }

    /// Return default LogLevel
    pub fn get_default() -> LogLevel {
        LogLevel::Info
    }
}

#[derive(Clone)]
pub struct Logger {
    pub log_level: LogLevel,
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "log_level: {}", self.log_level.to_str())
    }
}

impl Logger {
    /// Generate "error" level output with the specified message
    pub fn error(&self, message: String) {
        self.output(message, LogLevel::Error);
    }

    /// Generate "warning" level output with the specified message
    pub fn warning(&self, message: String) {
        self.output(message, LogLevel::Warning);
    }

    /// Generate "info" level output with the specified message
    pub fn info(&self, message: String) {
        self.output(message, LogLevel::Info);
    }

    /// Generate "debug" level output with the specified message
    pub fn debug(&self, message: String) {
        self.output(message, LogLevel::Debug);
    }

    /// Generic implementation for the output builder
    fn output(&self, message: String, message_level: LogLevel) {
        if &self.log_level.to_number() > &message_level.to_number() {
            return;
        }

        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        println!("+{} [{}] {}", timestamp, message_level.to_str(), message)
    }
}

#[cfg(test)]
mod tests {
    use output::{LogLevel};

    #[test]
    fn log_level_values() {
        assert_eq!(LogLevel::Error.to_number(), LogLevel::Error.to_number());
        assert_eq!(LogLevel::Warning.to_number(), LogLevel::Warning.to_number());
        assert_eq!(LogLevel::Info.to_number(), LogLevel::Info.to_number());
        assert_eq!(LogLevel::Debug.to_number(), LogLevel::Debug.to_number());
        assert!(LogLevel::Error.to_number() > LogLevel::Warning.to_number());
        assert!(LogLevel::Warning.to_number() > LogLevel::Info.to_number());
        assert!(LogLevel::Info.to_number() > LogLevel::Debug.to_number());
    }
}
