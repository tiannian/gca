use alloc::vec::Vec;
use primitive_types::{H160, H256};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHash(pub H256);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Txhash(pub H256);

#[derive(Debug, Default)]
pub struct MemoOperation(pub u32);

#[derive(Debug, Default)]
pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputOperation(pub u32);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(pub u64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MerkleHash(pub H256);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NodeAddress(pub H160);
