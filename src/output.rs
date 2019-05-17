use std::time::SystemTime;

enum LogLevel {
    Error,
    Warning,
    Info,
}

/// Generate "error" level output with the specified message
pub fn error(message: String)
{
    output(message, LogLevel::Error);
}

/// Generate "warning" level output with the specified message
pub fn warning(message: String)
{
    output(message, LogLevel::Warning);
}

/// Generate "info" level output with the specified message
pub fn info(message: String)
{
    output(message, LogLevel::Info);
}

/// Generic implementation for the output builder
fn output(message: String, level: LogLevel)
{
    let level_test = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warning => "WARNING",
        LogLevel::Info => "INFO",
    };

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    println!("+{} [{}] {}", timestamp, level_test, message)
}
