#![no_std]

#[link(wasm_import_module = "_gca_receipt_data")]
extern "C" {
    fn _set_data(
        data_ptr: *const u8,
        data_len: usize,
    );
}

pub fn set_data(data: &[u8]) {
    let data_ptr = data.as_ptr();
    let data_len = data.len();

    unsafe {
        _set_data(
            data_ptr, data_len,
        );
    }
}
