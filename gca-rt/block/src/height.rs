use gca_core::{BlockHash, BlockHeight};
use primitive_types::H256;

extern "C" {
    // Only get current chain's hash.
    fn _block_get_hash_by_height(height: i64, hash: *mut u8);

    fn _block_get_latest_block_hash(hash: *mut u8);
}

pub fn get_hash_by_height(height: BlockHeight) -> BlockHash {
    let mut hash = H256::default();

    unsafe { _block_get_hash_by_height(height.0, hash.as_mut_ptr()) };

    BlockHash(hash)
}

pub fn get_latest_hash() -> BlockHash {
    let mut hash = H256::default();

    unsafe { _block_get_latest_block_hash(hash.as_mut_ptr()) };

    BlockHash(hash)
}
