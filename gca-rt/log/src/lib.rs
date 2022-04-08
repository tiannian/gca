#![no_std]

mod utils;

use cstr_core::c_char;
use log::{Level, LevelFilter, SetLoggerError};

extern "C" {
    fn _gca_log(
        // level
        level: u8,
        // target
        target_ptr: *const c_char,
        // file
        file_ptr: *const c_char,
        // line
        line: u32,
        // messga
        message_ptr: *const c_char,
    );
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        let level = utils::level_to_u8(&record.level());
        let target_ptr = utils::str_to_ptr(record.target());
        let file_ptr = utils::opt_str_to_ptr(record.file());
        let line = record.line().unwrap_or_default();
        let message_ptr = utils::opt_str_to_ptr(record.args().as_str());

        unsafe {
            _gca_log(level, target_ptr, file_ptr, line, message_ptr);
        }
    }

    fn flush(&self) {}
}
