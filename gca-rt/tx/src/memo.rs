use alloc::vec::Vec;
use gca_core::{Memo, MemoOperation};

extern "C" {
    fn _memo_count() -> usize;

    fn _memo_get_operation_by_index(idx: usize) -> u32;

    fn _memo_get_data_len_by_index(idx: usize) -> usize;

    fn _memo_get_data_by_index(idx: usize, data_ptr: *mut u8);
}

pub fn get_memo_count() -> usize {
    unsafe { _memo_count() }
}

pub fn get_operation(idx: usize) -> MemoOperation {
    let ty = unsafe { _memo_get_operation_by_index(idx) };

    MemoOperation(ty)
}

pub fn get_data(idx: usize) -> Vec<u8> {
    let len = unsafe { _memo_get_data_len_by_index(idx) };

    let mut res = Vec::with_capacity(len);

    unsafe {
        _memo_get_data_by_index(idx, res.as_mut_ptr());
        res.set_len(len);
    }

    res
}

pub fn get_memos() -> Vec<Memo> {
    let mut memos = Vec::new();

    let len = get_memo_count();

    for idx in 0..len {
        let operation = get_operation(idx);
        let data = get_data(idx);

        memos.push(Memo { operation, data });
    }

    memos
}
