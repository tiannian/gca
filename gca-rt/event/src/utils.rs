use cstr_core::{c_char, CString};

use core::ptr;

pub fn str_to_ptr(s: &str) -> *const c_char {
    if let Ok(cstring) = CString::new(s) {
        cstring.as_ptr()
    } else {
        ptr::null()
    }
}
