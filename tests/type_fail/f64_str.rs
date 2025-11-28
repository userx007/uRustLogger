use logger::*;

fn main() {
    log_init!(LogLevel::Info, LogLevel::Info, false, false, false, false);

    // ❌ Trying to log a string as f64 should fail at compile time
    log_print!(
        LogLevel::Info,
        log_f64!("Hello")
    );
}
