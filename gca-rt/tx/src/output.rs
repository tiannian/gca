use alloc::vec::Vec;
use gca_core::{Amount, AssetType, Output, OutputData, OutputId, OutputOperation};

extern "C" {
    fn _output_get_count() -> usize;

    fn _output_get_operation_by_index(idx: usize) -> u32;

    fn _output_get_locker_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _output_get_verifier_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _output_is_token_by_index(idx: usize) -> bool;

    fn _output_get_token_by_index(idx: usize, asset_type: *mut u8) -> u64;

    fn _output_get_data_len_by_index(idx: usize) -> usize;

    fn _output_get_data_by_index(idx: usize, data_ptr: *mut u8);
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

pub fn get_verifier(idx: usize) -> OutputId {
    let mut output_id = OutputId::default();

    let txhash = &mut output_id.txhash.0 .0;

    let n = unsafe { _output_get_verifier_by_index(idx, txhash as *mut u8) };

    output_id.n = n;

    output_id
}

pub fn is_token(idx: usize) -> bool {
    unsafe { _output_is_token_by_index(idx) }
}

pub fn get_token(idx: usize) -> (AssetType, Amount) {
    let mut asset = AssetType::default();

    let asset_type = &mut asset.0 .0;

    let amount = unsafe { _output_get_token_by_index(idx, asset_type as *mut u8) };

    (asset, Amount(amount))
}

pub fn get_data(idx: usize) -> Vec<u8> {
    let len = unsafe { _output_get_data_len_by_index(idx) };

    let mut data = Vec::with_capacity(len);

    if len != 0 {
        unsafe {
            data.set_len(len);
            _output_get_data_by_index(idx, data.as_mut_ptr())
        }
    }

    data
}

pub fn get_output_data(idx: usize) -> OutputData {
    if is_token(idx) {
        let (a, b) = get_token(idx);
        OutputData::Token(a, b)
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

        let output = Output {
            data,
            locker,
            verifier,
            operation,
        };

        outputs.push(output);
    }

    outputs
}
