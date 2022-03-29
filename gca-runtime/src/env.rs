use gca_core::{Block, BlockHash, BlockHeight, Output, OutputId};

pub trait BlockchainEnv {
    fn get_block(&self, height: BlockHeight) -> Block;

    fn get_block_by_hash(&self, hash: BlockHash) -> BlockHeight;
}

pub trait UnspentOutputEnv {
    fn get_output(&self, idx: OutputId) -> Output;
}
