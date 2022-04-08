#![no_std]

mod utils;

use log::{Level, LevelFilter, SetLoggerError};

#[link(wasm_import_module = "_gca_log")]
extern "C" {
    fn _gca_log(
        // level
        level: u8,
        // target
        target_ptr: *const u8,

        target_len: usize,
        // file
        file_ptr: *const u8,

        file_len: usize,
        // line
        line: u32,
        // messga
        message_ptr: *const u8,

        message_len: usize,
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
        let target = utils::str_to_ptr(record.target());
        let file = utils::opt_str_to_ptr(record.file());
        let line = record.line().unwrap_or_default();
        let message = utils::opt_str_to_ptr(record.args().as_str());

        unsafe {
            _gca_log(
                level, target.0, target.1, file.0, file.1, line, message.0, message.1,
            );
        }
    }

    fn flush(&self) {}
}
