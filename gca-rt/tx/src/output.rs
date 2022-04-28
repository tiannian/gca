use alloc::vec::Vec;
use gca_core::{Amount, Output, OutputData, OutputId, OutputOperation, OutputCore};

extern "C" {
    fn _output_get_count() -> usize;

    fn _output_get_operation_by_index(idx: usize) -> u32;

    fn _output_get_locker_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _output_get_verifier_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _output_is_some_verifier_by_index(idx: usize) -> bool;

    fn _output_is_token_by_index(idx: usize) -> bool;

    fn _output_get_token_by_index(idx: usize) -> u64;

    fn _output_get_data_by_index(idx: usize, data_ptr: *mut usize) -> *const u8;
}

pub fn get_count() -> usize {
    unsafe { _output_get_count() }
}

pub fn get_operation(idx: usize) -> OutputOperation {
    let ty = unsafe { _output_get_operation_by_index(idx) };

    OutputOperation(ty)
}

pub fn get_locker(idx: usize) -> OutputId {
    let mut output_id = OutputId::default();

    let txhash = &mut output_id.txhash.0 .0;

    let n = unsafe { _output_get_locker_by_index(idx, txhash as *mut u8) };

    output_id.n = n;

    output_id
}

pub fn get_verifier(idx: usize) -> Option<OutputId> {
    let is_some = unsafe { _output_is_some_verifier_by_index(idx) };

    if is_some {
        let mut output_id = OutputId::default();

        let txhash = &mut output_id.txhash.0 .0;

        let n = unsafe { _output_get_verifier_by_index(idx, txhash as *mut u8) };

        output_id.n = n;

        Some(output_id)
    } else {
        None
    }
}

pub fn is_token(idx: usize) -> bool {
    unsafe { _output_is_token_by_index(idx) }
}

pub fn get_token(idx: usize) -> Amount {
    let amount = unsafe { _output_get_token_by_index(idx) };

    Amount(amount)
}

pub fn get_data(idx: usize) -> Vec<u8> {
    let mut len = 0usize;

    let ds = unsafe {
        let data = _output_get_data_by_index(idx, &mut len as *mut usize);
        core::slice::from_raw_parts(data, len)
    };

    Vec::from(ds)
}

pub fn get_output_data(idx: usize) -> OutputData {
    if is_token(idx) {
        let b = get_token(idx);
        OutputData::NativeToken(b)
    } else {
        OutputData::Data(get_data(idx))
    }
}

pub fn get_outputs() -> Vec<Output> {
    let size = get_count();

    let mut outputs = Vec::with_capacity(size);

    for idx in 0..size {
        let data = get_output_data(idx);
        let locker = get_locker(idx);
        let verifier = get_verifier(idx);
        let operation = get_operation(idx);

        let core = OutputCore {
            data,
            locker,
            verifier,
        };

        let output = Output {
            core,
            operation,
        };

        outputs.push(output);
    }

    outputs
}
