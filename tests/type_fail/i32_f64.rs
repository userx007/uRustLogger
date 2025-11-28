use logger::*;

fn main() {
    log_init!(LogLevel::Info, LogLevel::Info, false, false, false, false);

    // ❌ Trying to log a f64 as i32 should fail at compile time
    log_print!(
        LogLevel::Info,
        log_i32!(3.14)
    );
}
