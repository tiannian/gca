use gca_core::{Block, BlockHash, BlockHeight};

use crate::get_block;

extern "C" {
    fn _block_get_height_by_hash(hash: *const u8) -> i64;

    fn _block_get_pending_block_height() -> i64;

    fn _block_get_latest_block_height() -> i64;
}

pub fn get_height_by_hash(hash: BlockHash) -> BlockHeight {
    let height = unsafe { _block_get_height_by_hash(hash.0.as_ptr()) };

    BlockHeight(height)
}

pub fn get_block_by_hash(hash: BlockHash) -> Block {
    let height = get_height_by_hash(hash);

    get_block(height)
}

pub fn get_pending_height() -> BlockHeight {
    let height = unsafe { _block_get_pending_block_height() };

    BlockHeight(height)
}

pub fn get_pending_block() -> Block {
    let height = get_pending_height();

    get_block(height)
}

pub fn get_latest_height() -> BlockHeight {
    let height = unsafe { _block_get_latest_block_height() };

    BlockHeight(height)
}

pub fn get_latest_block() -> Block {
    let height = get_latest_height();

    get_block(height)
}
