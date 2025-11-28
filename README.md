# uRustLogger

`uRustLogger` is a lightweight, flexible, and fully-featured logging library for Rust, designed for both console and file output. It supports multiple log levels, colored console output, file logging with optional icons, and a rich set of macros for all common data types. The library is ideal for applications that need structured logging with minimal setup.

## Features

- **Multiple log levels**:
  `Verbose`, `Debug`, `Info`, `Warning`, `Error`, `Fatal`, `Fixed`

- **Console output with colors**:
  Each log level can be shown in a distinct color for easy readability.

- **File logging**:
  Logs can be written to a timestamped file. Optionally, each log level can include Unicode emoji icons instead of plain text.

- **Configurable formatting**:
  - Include/exclude timestamps
  - Enable/disable console colors
  - Use icons or plain text in log files

- **Rich macro-based API**:
  - `log_print!` – log multiple values at once
  - Type-specific macros for all Rust primitive types:
    - Strings: `log_str!`
    - Integers: `log_i8!`, `log_i16!`, `log_i32!`, `log_i64!`
    - Unsigned: `log_u8!`, `log_u16!`, `log_u32!`, `log_u64!`
    - Floats: `log_f32!`, `log_f64!`
    - Booleans: `log_bool!`
    - Characters: `log_char!`
    - Hex: `log_hex8!`, `log_hex16!`, `log_hex32!`, `log_hex64!`
    - Pointers: `log_ptr!`

- **Global thread-safe logger** using `Arc<Mutex<Logger>>`

- **Initialization macros**:
  `log_init!` to configure the logger and `log_deinit!` to safely shut it down.

## Quick Setup

```rust
use logger::*;

fn main() {
    log_init!(
        LogLevel::Verbose, // console threshold
        LogLevel::Verbose, // file threshold
        true,              // enable file logging
        true,              // enable colors
        true,              // include date
        true               // use icons in file
    );

    log_print!(LogLevel::Info, log_str!("Application started"), log_i32!(123));
    log_print!(LogLevel::Debug, log_str!("Debug value:"), log_f64!(3.1415));

    let value = 42;
    log_print!(LogLevel::Verbose, log_str!("Pointer:"), log_ptr!(&value));

    log_deinit!();
}
