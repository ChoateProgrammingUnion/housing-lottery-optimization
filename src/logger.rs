extern crate chrono;
extern crate env_logger;
extern crate log;
use self::chrono::Local;
use self::env_logger::Builder;
use std::io::Write;

#[macro_export]
macro_rules! log_error {
    ($text:expr) => {
        log::error!("{}", $text)
    };
    ($text:expr, $header:expr) => {
        log::error!("{}[{}]{} {}", "\x1B[93m", $header, "\x1B[0m", $text)
    };
}

#[macro_export]
macro_rules! log_info {
    ($text:expr) => {
        log::info!("{}", $text)
    };
    ($text:expr, $header:expr) => {
        log::info!("{}[{}]{} {}", "\x1B[93m", $header, "\x1B[0m", $text)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($text:expr) => {
        log::debug!("{}", $text)
    };
    ($text:expr, $header:expr) => {
        log::debug!("{}[{}]{} {}", "\x1B[93m", $header, "\x1B[0m", $text)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($text:expr) => {
        log::trace!("{}", $text)
    };
    ($text:expr, $header:expr) => {
        log::trace!("{}[{}]{} {}", "\x1B[93m", $header, "\x1B[0m", $text)
    };
}

pub fn init(log_level: log::LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}{}{} {}[{}]{} {}",
                "\x1B[91m",
                Local::now().format("%Y-%m-%d:%H:%M:%S"),
                "\x1B[0m",
                "\x1B[95m",
                record.level(),
                "\x1B[0m",
                record.args()
            )
        })
        .filter(None, log_level)
        .init();
}
