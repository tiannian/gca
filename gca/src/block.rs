use alloc::string::String;

use crate::{BlockHeight, MerkleHash, Timestamp, NodeAddress, BlockHash};

pub struct BlockHeader {
    pub chain_id: String,
    pub height: BlockHeight,
    pub time: Option<Timestamp>,
    pub txs_hash: MerkleHash,
    pub app_hash: MerkleHash,
    pub proposer: NodeAddress,
}

pub struct Block {
    pub hash: BlockHash,
    pub header: BlockHeader,
}
