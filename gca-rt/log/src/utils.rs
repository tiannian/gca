use log::Level;

use core::ptr;

pub fn level_to_u8(level: &Level) -> u8 {
    match level {
        Level::Error => 1,
        Level::Warn => 2,
        Level::Info => 3,
        Level::Debug => 4,
        Level::Trace => 5,
    }
}

pub fn str_to_ptr(s: &str) -> (*const u8, usize) {
    (s.as_ptr(), s.len())
}

pub fn opt_str_to_ptr(s: Option<&str>) -> (*const u8, usize) {
    if let Some(s) = s {
        (s.as_ptr(), s.len())
    } else {
        (ptr::null(), 0)
    }
}
