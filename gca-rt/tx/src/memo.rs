use alloc::vec::Vec;
use gca_core::{Memo, MemoOperation};

extern "C" {
    fn _memo_count() -> usize;

    fn _memo_get_operation_by_index(idx: usize) -> u32;

    fn _memo_get_data_by_index(idx: usize, len: *mut usize) -> *const u8;
}

pub fn get_memo_count() -> usize {
    unsafe { _memo_count() }
}

pub fn get_operation(idx: usize) -> MemoOperation {
    let ty = unsafe { _memo_get_operation_by_index(idx) };

    MemoOperation(ty)
}

pub fn get_data(idx: usize) -> Vec<u8> {
    let mut len = 0usize;

    let ds = unsafe {
        let ptr = _memo_get_data_by_index(idx, &mut len as *mut usize);
        core::slice::from_raw_parts(ptr, len)
    };

    Vec::from(ds)
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
