use alloc::vec::Vec;
use primitive_types::H256;

#[derive(Debug, Default)]
pub struct Txhash(pub H256);

pub struct MemoOperation(pub u64);

pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

pub struct OutputOperation(pub u32);

