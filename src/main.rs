use logger::*;

fn main() {
    // Initialize the logger:
    // console threshold = Verbose
    // file threshold = Verbose
    // enable file logging = true
    // enable colors = true
    // include date = true
    // use icons in file = true
    log_init!(
        LogLevel::Verbose, // console threshold
        LogLevel::Verbose, // file threshold
        true,              // enable file logging
        true,              // enable colors
        true,              // include date
        true               // use icons in file
    );

    // --- Basic string, integer, bool ---
    log_print!(
        LogLevel::Fixed,
        log_str!("Starting application"),
        log_i32!(123),
        log_bool!(true)
    );

    // --- Pointer logging ---
    let value = 999;
    log_print!(
        LogLevel::Debug,
        log_str!("Value address:"),
        log_ptr!(&value)
    );

    // --- Hex logging ---
    log_print!(
        LogLevel::Verbose,
        log_hex8!(0xABu8),
        log_hex16!(4444u16),
        log_hex32!(0xDEADBEEFu32),
        log_hex64!(0xCAFEBABEDEADC0DEu64)
    );

    // --- Floating point ---
    log_print!(
        LogLevel::Info,
        log_str!("Pi approximation:"),
        log_f32!(3.1415),
        log_str!("..and e approximation:"),
        log_f64!(2.718281828)
    );

    // --- All integer types ---
    log_print!(
        LogLevel::Debug,
        log_i8!(-8),
        log_i16!(-16),
        log_i32!(-32),
        log_i64!(-64),
        log_u8!(8),
        log_u16!(16),
        log_u32!(32),
        log_u64!(64)
    );

    // --- Char logging ---
    log_print!(
        LogLevel::Info,
        log_str!("Char:"),
        log_char!('X'),
        log_char!('✔')
    );

    // --- Error example ---
    log_print!(
        LogLevel::Error,
        log_str!("This is an error caused by the value"),
        log_f64!(3.1415926535)
    );

    log_print!(
        LogLevel::Fatal,
        log_str!("and this is a fatal one.."),
        log_f64!(3.1415926535)
    );

    log_print!(LogLevel::Fixed, log_str!("Ending application..."));

    // --- Show file location ---
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
