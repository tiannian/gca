use alloc::vec::Vec;
use gca_core::{Input, InputOperation, OutputId};

extern "C" {
    fn _input_get_count() -> usize;

    fn _input_get_operation_by_index(idx: usize) -> u32;

    fn _input_is_reference_input(idx: usize) -> bool;

    fn _input_get_output_id_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _input_get_unlock_data_len_by_index(idx: usize) -> usize;

    fn _input_get_unlock_data_by_index(idx: usize, data_ptr: *mut u8);
}

pub fn get_operation(idx: usize) -> InputOperation {
    let is_ref = unsafe { _input_is_reference_input(idx) };

    if is_ref {
        InputOperation::Reference
    } else {
        let t = unsafe { _input_get_operation_by_index(idx) };
        InputOperation::Input(t)
    }
}

pub fn get_unlock_data(idx: usize) -> Vec<u8> {
    let len = unsafe { _input_get_unlock_data_len_by_index(idx) };

    let mut res: Vec<u8> = Vec::with_capacity(len);

    unsafe {
        res.set_len(len);
        _input_get_unlock_data_by_index(idx, res.as_mut_ptr())
    }

    res
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
