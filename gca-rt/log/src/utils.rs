use cstr_core::{c_char, CString};
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

pub fn str_to_ptr(s: &str) -> *const c_char {
    if let Ok(cstring) = CString::new(s) {
        cstring.as_ptr()
    } else {
        ptr::null()
    }
}

pub fn opt_str_to_ptr(s: Option<&str>) -> *const c_char {
    if let Some(st) = s {
        if let Ok(cstring) = CString::new(st) {
            return cstring.as_ptr();
        }
    }
    ptr::null()
}
