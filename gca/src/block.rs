use crate::{BlockHash, BlockHeight, MerkleHash, NodeAddress, Timestamp};

pub struct BlockHeader {
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
