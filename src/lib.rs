// Cargo.toml dependencies:
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
            LogLevel::Verbose => "\x1b[90m",   // Bright black / gray
            LogLevel::Debug   => "\x1b[96m",   // Bright cyan
            LogLevel::Info    => "\x1b[92m",   // Bright green
            LogLevel::Warning => "\x1b[93m",   // Bright yellow
            LogLevel::Error   => "\x1b[91m",   // Bright red
            LogLevel::Fatal   => "\x1b[95m",   // Bright magenta
            LogLevel::Fixed   => "\x1b[97m",   // Bright white
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            LogLevel::Verbose => "💬",
            LogLevel::Debug => "🐞",
            LogLevel::Info => "ℹ️",
            LogLevel::Warning => "⚠️",
            LogLevel::Error => "❌",
            LogLevel::Fatal => "💀",
            LogLevel::Fixed => "✅",
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
    pub use_icons_in_file: bool,
    log_file: Option<std::fs::File>,
    pub log_file_path: Option<String>,
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
            use_icons_in_file: false,
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

        // Console
        if self.current_level >= self.console_threshold {
            let msg = format!("{}{} | {}\n", timestamp, self.current_level, self.buffer);
            if self.use_colors {
                print!("{}{}{}\x1b[0m", self.current_level.color(), msg, "");
            } else {
                print!("{}", msg);
            }
        }

        // File
        if self.file_logging_enabled {
            if let Some(file) = &mut self.log_file {
                let level_repr = if self.use_icons_in_file {
                    self.current_level.icon()
                } else {
                    &self.current_level.to_string()
                };
                let file_message = format!("{}{} | {}\n", timestamp, level_repr, self.buffer);
                let _ = file.write_all(file_message.as_bytes());
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

// ---------- Type-safe Macros ----------
#[macro_export]
macro_rules! log_str {
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
macro_rules! log_ptr {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append(format!("{:p}", $v));
        }
    };
}
#[macro_export]
macro_rules! log_char {
    ($v:expr) => {
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    };
}

// Hexadecimal types
#[macro_export]
macro_rules! log_hex8 {
    ($v:expr) => {{
        fn _f(_: u8) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    }};
}
#[macro_export]
macro_rules! log_hex16 {
    ($v:expr) => {{
        fn _f(_: u16) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    }};
}
#[macro_export]
macro_rules! log_hex32 {
    ($v:expr) => {{
        fn _f(_: u32) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    }};
}
#[macro_export]
macro_rules! log_hex64 {
    ($v:expr) => {{
        fn _f(_: u64) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append_hex($v);
        }
    }};
}

// Signed integers
#[macro_export]
macro_rules! log_i8 {
    ($v:expr) => {{
        fn _f(_: i8) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_i16 {
    ($v:expr) => {{
        fn _f(_: i16) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_i32 {
    ($v:expr) => {{
        fn _f(_: i32) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_i64 {
    ($v:expr) => {{
        fn _f(_: i64) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}

// Unsigned integers
#[macro_export]
macro_rules! log_u8 {
    ($v:expr) => {{
        fn _f(_: u8) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_u16 {
    ($v:expr) => {{
        fn _f(_: u16) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_u32 {
    ($v:expr) => {{
        fn _f(_: u32) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_u64 {
    ($v:expr) => {{
        fn _f(_: u64) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}

// Floating point
#[macro_export]
macro_rules! log_f32 {
    ($v:expr) => {{
        fn _f(_: f32) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}
#[macro_export]
macro_rules! log_f64 {
    ($v:expr) => {{
        fn _f(_: f64) {}
        _f($v);
        |logger: &mut $crate::Logger| {
            logger.append($v);
        }
    }};
}

// ---------- Main print macro ----------
#[macro_export]
macro_rules! log_print {
    ($level:expr, $($val:expr),+ $(,)?) => {{
        let mut logger = $crate::LOGGER.lock().unwrap();
        logger.set_level($level);
        $( $val(&mut logger); )+
        logger.print();
    }};
}

// ---------- Init/Deinit ----------
#[macro_export]
macro_rules! log_init {
    ($console:expr, $file:expr, $enable_file:expr, $enable_colors:expr, $include_date:expr, $use_icons:expr) => {{
        let mut logger = $crate::LOGGER.lock().unwrap();
        logger.set_console_threshold($console);
        logger.set_file_threshold($file);
        logger.use_colors = $enable_colors;
        logger.include_date = $include_date;
        logger.use_icons_in_file = $use_icons;
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

// ---------- Tests ----------
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
        logger.use_icons_in_file = false;
    }

    #[test]
    fn test_basic_logging() {
        reset_logger();
        log_print!(
            LogLevel::Info,
            log_str!("Hello"),
            log_i32!(42),
            log_bool!(true)
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_hex_logging() {
        reset_logger();
        log_print!(
            LogLevel::Debug,
            log_hex8!(0xABu8),
            log_hex16!(0x1234u16),
            log_hex32!(0xDEADBEEFu32),
            log_hex64!(0xDEADBEEFFEEDC0DEu64)
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_pointer_logging() {
        reset_logger();
        let x = 123u32;
        log_print!(LogLevel::Info, log_ptr!(&x));
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
        log_print!(
            LogLevel::Info,
            log_str!("File logging test"),
            log_i32!(2025)
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.file_logging_enabled);
        if let Some(path) = &logger.log_file_path {
            assert!(Path::new(path).exists());
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
        log_print!(LogLevel::Info, log_str!("This should not print"));
        log_print!(LogLevel::Error, log_str!("This should print"));
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_logging_all_types() {
        reset_logger();
        let x = 123;
        log_print!(
            LogLevel::Info,
            log_str!("Hello"),
            log_i8!(1),
            log_i16!(2),
            log_i32!(42),
            log_i64!(12345),
            log_u8!(0xAB),
            log_u16!(0x1234u16),
            log_u32!(0xDEADBEEFu32),
            log_u64!(0xDEADBEEFFEEDC0DEu64),
            log_f32!(3.14),
            log_f64!(2.71828),
            log_bool!(true),
            log_ptr!(&x),
            log_char!('X')
        );
        let logger = LOGGER.lock().unwrap();
        assert!(logger.buffer.is_empty());
    }

    #[test]
    fn test_file_logging_with_icons() {
        reset_logger();
        {
            let mut logger = LOGGER.lock().unwrap();
            logger.use_icons_in_file = true;
            logger.enable_file_logging();
        }
        log_print!(LogLevel::Warning, log_str!("Warning with icon"));
        let logger = LOGGER.lock().unwrap();
        assert!(logger.file_logging_enabled);
        if let Some(path) = &logger.log_file_path {
            assert!(Path::new(path).exists());
            let _ = std::fs::remove_file(path);
        }
    }
}
