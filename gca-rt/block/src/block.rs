use gca_core::{Block, BlockHash, BlockHeader, BlockHeight, MerkleHash, NodeAddress, Timestamp};
use primitive_types::H256;

extern "C" {
    fn _block_get_hash_by_height(height: i64, hash: *mut u8);

    fn _block_get_timestamp_by_height(height: i64) -> i64;

    fn _block_get_txs_hash_by_height(height: i64, hash: *mut u8);

    fn _block_get_app_hash_by_height(height: i64, hash: *mut u8);

    fn _block_get_proposer_by_height(height: i64, addr: *mut u8);
}

pub fn get_block(height: BlockHeight) -> Block {
    let h = height.0;

    let mut hash = H256::default();

    unsafe {
        _block_get_hash_by_height(h, hash.as_mut_ptr());
    }

    let ts = unsafe { _block_get_timestamp_by_height(h) };

    let time = if ts == 0 { None } else { Some(Timestamp(ts)) };

    let mut app_hash = H256::default();

    unsafe { _block_get_app_hash_by_height(h, app_hash.as_mut_ptr()) }

    let mut txs_hash = H256::default();

    unsafe { _block_get_txs_hash_by_height(h, txs_hash.as_mut_ptr()) }

    let proposer = NodeAddress::default();

    let header = BlockHeader {
        height,
        time,
        app_hash: MerkleHash(app_hash),
        txs_hash: MerkleHash(txs_hash),
        proposer,
    };

    Block {
        hash: BlockHash(hash),
        header,
    }
}
