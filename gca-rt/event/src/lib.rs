#![no_std]

use cstr_core::c_char;

mod utils;

extern "C" {
    fn _emit(
        name_ptr: *const c_char,
        key_ptr: *const u8,
        key_len: usize,
        val_ptr: *const u8,
        val_len: usize,
    );
}

pub fn emit(name: &str, key: &[u8], value: &[u8]) {
    let name_ptr = utils::str_to_ptr(name);
    let key_ptr = key.as_ptr();
    let key_len = key.len();

    let val_ptr = value.as_ptr();
    let val_len = value.len();

    unsafe {
        _emit(name_ptr, key_ptr, key_len, val_ptr, val_len);
    }
}
