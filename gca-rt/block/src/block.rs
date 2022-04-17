use gca_core::{Block, BlockHash, BlockHeader, BlockHeight, MerkleHash, NodeAddress, Timestamp};
use primitive_types::{H160, H256};

extern "C" {
    fn _block_get_height_by_hash(hash: *const u8) -> i64;

    fn _block_get_timestamp_by_hash(hash: *const u8) -> i64;

    fn _block_get_parent_hash_by_hash(hash: *const u8, parent: *mut u8);

    fn _block_get_txs_hash_by_hash(hash: *const u8, txs: *mut u8);

    fn _block_get_app_hash_by_hash(hash: *const u8, app: *mut u8);

    fn _block_get_proposer_by_hash(hash: *const u8, addr: *mut u8);
}

pub fn get_block(hash: BlockHash) -> Block {
    let h = hash.0.as_ptr();

    let height = unsafe { _block_get_height_by_hash(h) };

    let ts = unsafe { _block_get_timestamp_by_hash(h) };

    let time = if ts == 0 { None } else { Some(Timestamp(ts)) };

    let mut parent = H256::default();

    unsafe { _block_get_parent_hash_by_hash(h, parent.as_mut_ptr()) };

    let mut app_hash = H256::default();

    unsafe { _block_get_app_hash_by_hash(h, app_hash.as_mut_ptr()) }

    let mut txs_hash = H256::default();

    unsafe { _block_get_txs_hash_by_hash(h, txs_hash.as_mut_ptr()) }

    let mut proposer = H160::default();

    unsafe { _block_get_proposer_by_hash(h, proposer.as_mut_ptr()) };

    let header = BlockHeader {
        height: BlockHeight(height),
        parent_hash: BlockHash(parent),
        time,
        app_hash: MerkleHash(app_hash),
        txs_hash: MerkleHash(txs_hash),
        proposer: NodeAddress(proposer),
    };

    Block { hash, header }
}
