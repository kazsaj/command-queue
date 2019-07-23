use std::time::SystemTime;

#[derive(Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Clone)]
pub struct Logger {
    pub log_level: LogLevel,
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
        if Logger::log_level_to_number(&self.log_level)
            > Logger::log_level_to_number(&message_level)
        {
            return;
        }

        let level_output = match message_level {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        };

        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        println!("+{} [{}] {}", timestamp, level_output, message)
    }

    /// Convert the LogLevel enum to a numeric representation making it easier to compare them
    fn log_level_to_number(log_level: &LogLevel) -> usize {
        return match log_level {
            LogLevel::Error => 3,
            LogLevel::Warning => 2,
            LogLevel::Info => 1,
            LogLevel::Debug => 0,
        };
    }
}

#[cfg(test)]
mod tests {
    use output::{LogLevel, Logger};

    #[test]
    fn logger_values() {
        assert_eq!(
            Logger::log_level_to_number(&LogLevel::Error),
            Logger::log_level_to_number(&LogLevel::Error)
        );
        assert_eq!(
            Logger::log_level_to_number(&LogLevel::Warning),
            Logger::log_level_to_number(&LogLevel::Warning)
        );
        assert_eq!(
            Logger::log_level_to_number(&LogLevel::Info),
            Logger::log_level_to_number(&LogLevel::Info)
        );
        assert_eq!(
            Logger::log_level_to_number(&LogLevel::Debug),
            Logger::log_level_to_number(&LogLevel::Debug)
        );
        assert!(
            Logger::log_level_to_number(&LogLevel::Error)
                > Logger::log_level_to_number(&LogLevel::Warning)
        );
        assert!(
            Logger::log_level_to_number(&LogLevel::Warning)
                > Logger::log_level_to_number(&LogLevel::Info)
        );
        assert!(
            Logger::log_level_to_number(&LogLevel::Info)
                > Logger::log_level_to_number(&LogLevel::Debug)
        );
    }
}
