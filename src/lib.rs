// Cargo.toml must include:
// chrono = "0.4"
// lazy_static = "1.4"

use chrono::Local;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum LogLevel {
    Verbose,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
    Fixed,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogLevel::Verbose => "VERBOSE",
            LogLevel::Debug => "  DEBUG",
            LogLevel::Info => "   INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "  ERROR",
            LogLevel::Fatal => "  FATAL",
            LogLevel::Fixed => "  FIXED",
        };
        write!(f, "{}", s)
    }
}

impl LogLevel {
    fn color(&self) -> &'static str {
        match self {
            LogLevel::Verbose => "\x1b[90m", // Bright Black
            LogLevel::Debug => "\x1b[36m",   // Cyan
            LogLevel::Info => "\x1b[32m",    // Green
            LogLevel::Warning => "\x1b[33m", // Yellow
            LogLevel::Error => "\x1b[31m",   // Red
            LogLevel::Fatal => "\x1b[91m",   // Bright Red
            LogLevel::Fixed => "\x1b[97m",   // Bright White
        }
    }
}

// ---------- Logger Struct ----------
pub struct Logger {
    buffer: String,
    current_level: LogLevel,
    pub console_threshold: LogLevel,
    pub file_threshold: LogLevel,
    pub file_logging_enabled: bool,
    pub use_colors: bool,
    pub include_date: bool,
    log_file: Option<std::fs::File>,
    pub log_file_path: Option<String>, // pub so tests can inspect
}

impl Logger {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            current_level: LogLevel::Info,
            console_threshold: LogLevel::Verbose,
            file_threshold: LogLevel::Verbose,
            file_logging_enabled: false,
            use_colors: true,
            include_date: true,
            log_file: None,
            log_file_path: None,
        }
    }

    pub fn append<T: std::fmt::Display>(&mut self, value: T) {
        self.buffer.push_str(&format!("{} ", value));
    }

    pub fn append_bool(&mut self, value: bool) {
        self.buffer.push_str(if value { "true " } else { "false " });
    }

    pub fn append_hex<T: std::fmt::UpperHex>(&mut self, value: T) {
        self.buffer.push_str(&format!("0x{:X} ", value));
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.current_level = LogLevel::Info;
    }

    fn timestamp(&self) -> String {
        let now = Local::now();
        if self.include_date {
            format!("{} | ", now.format("%Y-%m-%d %H:%M:%S%.6f"))
        } else {
            format!("{} | ", now.format("%H:%M:%S%.6f"))
        }
    }

    pub fn print(&mut self) {
        let timestamp = self.timestamp();
        let full_message = format!("{}{} | {}\n", timestamp, self.current_level, self.buffer);

        if self.current_level >= self.console_threshold {
            if self.use_colors {
                print!(
                    "{}{}{}\x1b[0m",
                    self.current_level.color(),
                    full_message,
                    ""
                );
            } else {
                print!("{}", full_message);
            }
        }

        if self.file_logging_enabled {
            if let Some(file) = &mut self.log_file {
                let _ = file.write_all(full_message.as_bytes());
            }
        }

        self.reset();
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.current_level = level;
    }

    pub fn set_console_threshold(&mut self, level: LogLevel) {
        self.console_threshold = level;
    }
    pub fn set_file_threshold(&mut self, level: LogLevel) {
        self.file_threshold = level;
    }

    pub fn enable_file_logging(&mut self) {
        if !self.file_logging_enabled {
            let filename = format!("log_{}.txt", chrono::Local::now().format("%Y%m%d_%H%M%S"));
            self.log_file = Some(
                std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&filename)
                    .unwrap(),
            );
            self.log_file_path = Some(filename);
            self.file_logging_enabled = true;
        }
    }

    pub fn disable_file_logging(&mut self) {
        self.log_file = None;
        self.file_logging_enabled = false;
        self.log_file_path = None;
    }
}

// ---------- Global Logger ----------
lazy_static::lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));
}

// ---------- Macros ----------
//
// The type-specific macros expand to closures of type `FnOnce(&mut Logger)`.
// That way log_print_multi! can accept them as expressions and call them after locking.
#[macro_export]
macro_rules! log_str {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    };
}
#[macro_export]
macro_rules! log_int {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    };
}
#[macro_export]
macro_rules! log_bool {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append_bool($v);
        }
    };
}
#[macro_export]
macro_rules! log_float {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    };
}
#[macro_export]
macro_rules! log_double {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    };
}
#[macro_export]
macro_rules! log_hex8 {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    };
}
#[macro_export]
macro_rules! log_hex16 {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    };
}
#[macro_export]
macro_rules! log_hex32 {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    };
}
#[macro_export]
macro_rules! log_hex64 {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    };
}
#[macro_export]
macro_rules! log_ptr {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append(format!("{:p}", $v));
        }
    };
}

// log_print_multi accepts one-or-more expressions (closures produced by the above macros)
#[macro_export]
macro_rules! log_print_multi {
    ($level:expr, $($val:expr),+ $(,)?) => {{
        let mut logger = $crate::LOGGER.lock().unwrap();
        logger.set_level($level);
        $(
            // each $val is a closure like |logger: &mut Logger| { ... }
            $val(&mut logger);
        )+
        logger.print();
    }};
}

// init/deinit macros (lock once)
#[macro_export]
macro_rules! log_init {
    ($console:expr, $file:expr, $enable_file:expr, $enable_colors:expr, $include_date:expr) => {{
        let mut logger = $crate::LOGGER.lock().unwrap();
        logger.set_console_threshold($console);
        logger.set_file_threshold($file);
        logger.use_colors = $enable_colors;
        logger.include_date = $include_date;
        if $enable_file {
            logger.enable_file_logging();
        } else {
            logger.disable_file_logging();
        }
    }};
}

#[macro_export]
macro_rules! log_deinit {
    () => {{
        let mut logger = $crate::LOGGER.lock().unwrap();
        logger.disable_file_logging();
    }};
}

// ---------------- Tests ----------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn reset_logger() {
        let mut logger = LOGGER.lock().unwrap();
        logger.disable_file_logging();
        logger.buffer.clear();
        logger.console_threshold = LogLevel::Verbose;
        logger.file_threshold = LogLevel::Verbose;
        logger.use_colors = false;
        logger.include_date = false;
    }

    #[test]
    fn test_basic_logging() {
        reset_logger();
        log_print_multi!(
            LogLevel::Info,
            log_str!("Hello"),
            log_int!(42),
            log_bool!(true),
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_hex_logging() {
        reset_logger();
        log_print_multi!(
            LogLevel::Debug,
            log_hex8!(0xABu8),
            log_hex16!(0x1234u16),
            log_hex32!(0xDEADBEEFu32),
            log_hex64!(0xDEADBEEFFEEDC0DEu64),
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_pointer_logging() {
        reset_logger();
        let x = 123u32;
        log_print_multi!(LogLevel::Info, log_ptr!(&x));
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_file_logging_creates_file() {
        reset_logger();
        {
            let mut logger = LOGGER.lock().unwrap();
            logger.enable_file_logging();
        }

        log_print_multi!(
            LogLevel::Info,
            log_str!("File logging test"),
            log_int!(2025)
        );

        let logger = LOGGER.lock().unwrap();
        assert!(logger.file_logging_enabled);
        if let Some(path) = &logger.log_file_path {
            assert!(Path::new(path).exists());
            // cleanup
            let _ = std::fs::remove_file(path);
        }
    }

    #[test]
    fn test_log_levels() {
        reset_logger();
        {
            let mut logger = LOGGER.lock().unwrap();
            logger.set_console_threshold(LogLevel::Error);
            logger.set_file_threshold(LogLevel::Warning);
        }

        log_print_multi!(LogLevel::Info, log_str!("This should not print"));
        log_print_multi!(LogLevel::Error, log_str!("This should print"));

        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }
}

