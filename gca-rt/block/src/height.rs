use gca_core::BlockHeight;
use primitive_types::H256;

extern "C" {
    fn _block_get_hash_by_height(hash: *const u8) -> u64;
}

pub fn get_hash(hash: H256) -> BlockHeight {
    BlockHeight(0)
}
