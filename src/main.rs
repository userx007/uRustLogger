use logger::*;

fn main() {
    // Initialize the logger:
    // console threshold = Info
    // file threshold = Verbose
    // enable file logging = true
    // enable colors = true
    // include date = true
    log_init!(
        LogLevel::Verbose,
        LogLevel::Verbose,
        true,   // enable file logging
        true,   // enable colors
        true    // include date
    );

    // Test: simple logging
    log_print_multi!(
        LogLevel::Info,
        log_str!("Starting application"),
        log_int!(123),
        log_bool!(true)
    );

    // Test: pointer logging
    let value = 999;
    log_print_multi!(
        LogLevel::Debug,
        log_str!("Value address:"),
        log_ptr!(&value)
    );

    // Test: hex logging
    log_print_multi!(
        LogLevel::Verbose,
        log_hex8!(0xABu8),
        log_hex16!(0x1234u16),
        log_hex32!(0xDEADBEEFu32),
        log_hex64!(0xCAFEBABEDEADC0DEu64)
    );

    // Test: floating point
    log_print_multi!(
        LogLevel::Info,
        log_str!("Pi approximation:"),
        log_double!(3.1415926535)
    );

    // Show file location
    {
        let logger = LOGGER.lock().unwrap();
        if let Some(path) = &logger.log_file_path {
            println!("Log file written to: {}", path);
        }
    }

    // Shut down logging
    log_deinit!();

    println!("Logger test complete.");
}
