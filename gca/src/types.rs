use alloc::vec::Vec;
use primitive_types::{H160, H256};

#[derive(Debug, Default)]
pub struct BlockHash(pub H256);

#[derive(Debug, Default)]
pub struct Txhash(pub H256);

#[derive(Debug, Default)]
pub struct MemoOperation(pub u32);

#[derive(Debug, Default)]
pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct AssetType(pub H160);

#[derive(Debug, Default)]
pub struct OutputOperation(pub u32);

#[derive(Debug, Default)]
pub struct Amount(pub u64);

#[derive(Debug, Default)]
pub struct BlockHeight(pub u64);

#[derive(Debug, Default)]
pub struct MerkleHash(pub H256);

#[derive(Debug, Default)]
pub struct Timestamp(i64);

#[derive(Debug, Default)]
pub struct NodeAddress(H160);
