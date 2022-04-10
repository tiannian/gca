#![no_std]

#[link(wasm_import_module = "_gca_log")]
extern "C" {
    fn _gca_emit(
        name_ptr: *const u8,
        name_len: usize,
        key_ptr: *const u8,
        key_len: usize,
        val_ptr: *const u8,
        val_len: usize,
        index: bool,
    );
}

pub fn emit(name: &str, key: &[u8], value: &[u8], index: bool) {
    let name_ptr = name.as_ptr();
    let name_len = name.len();

    let key_ptr = key.as_ptr();
    let key_len = key.len();

    let val_ptr = value.as_ptr();
    let val_len = value.len();

    unsafe {
        _gca_emit(name_ptr, name_len, key_ptr, key_len, val_ptr, val_len, index);
    }
}
