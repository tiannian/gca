use alloc::vec::Vec;

mod number;
pub use number::*;

mod hash;
pub use hash::*;

#[derive(Debug, Default, Clone)]
pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}
