use alloc::vec::Vec;
use primitive_types::{H256, H160};

#[derive(Debug, Default)]
pub struct Txhash(pub H256);

pub struct MemoOperation(pub u64);

pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

pub struct AssetType(pub H160);

pub struct OutputOperation(pub u32);

pub struct Amount(pub u64);

