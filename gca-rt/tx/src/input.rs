use alloc::{vec::Vec, string::String};
use cstr_core::CStr;
use gca_core::{Input, InputOperation, OutputId};

extern "C" {
    fn _input_get_count() -> usize;

    fn _input_get_operation_by_index(idx: usize) -> u32;

    fn _input_is_reference_input(idx: usize) -> bool;

    fn _input_get_reference_by_index(idx: usize) -> u32;

    fn _input_get_reference_name_by_index(idx: usize) -> *const i8;

    fn _input_get_output_id_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _input_get_unlock_data_by_index(idx: usize, data_len: *mut usize) -> *const u8;
}

pub fn get_operation(idx: usize) -> InputOperation {
    let is_ref = unsafe { _input_is_reference_input(idx) };

    if is_ref {
        let name_cstr = unsafe {
            let ptr = _input_get_reference_name_by_index(idx);

            CStr::from_ptr(ptr)
        };

        let t = unsafe { _input_get_reference_by_index(idx) };
        InputOperation::Reference(String::from(name_cstr.to_str().unwrap()), t)
    } else {
        let t = unsafe { _input_get_operation_by_index(idx) };
        InputOperation::Input(t)
    }
}

pub fn get_unlock_data(idx: usize) -> Vec<u8> {

    let mut len = 0usize;

    let ds = unsafe {
        let ptr = _input_get_unlock_data_by_index(idx, &mut len as *mut usize);
        core::slice::from_raw_parts(ptr, len)
    };

    Vec::from(ds)
}

pub fn get_output_id(idx: usize) -> OutputId {
    let mut oid = OutputId::default();

    let n = unsafe {
        let s = &mut oid.txhash.0 .0;

        _input_get_output_id_by_index(idx, s as *mut u8)
    };

    oid.n = n;

    oid
}

pub fn get_input_count() -> usize {
    unsafe { _input_get_count() }
}

pub fn get_inputs() -> Vec<Input> {
    let mut inputs = Vec::new();

    let count = get_input_count();

    for idx in 0..count {
        let operation = get_operation(idx);
        let output_id = get_output_id(idx);
        let unlock = get_unlock_data(idx);

        let input = Input {
            operation,
            output_id,
            unlock,
        };

        inputs.push(input);
    }

    inputs
}
