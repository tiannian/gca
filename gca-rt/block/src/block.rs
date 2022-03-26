extern "C" {
    fn _block_get_height_by_hash(height: u64, hash: *mut u8);

    fn _block_get_timestamp_by_hash(height: u64) -> i64;

    fn _block_get_txs_hash_by_hash(height: u64, hash: *mut u8);

    fn _block_get_app_hash_by_hash(height: u64, hash: *mut u8);

    fn _block_get_proposer_by_hash(height: u64, addr: *mut u8);
}

// pub fn get_hash(height: BlockHeight)

pub fn get_block() {}

pub fn get_block_by_hash() {}

pub fn get_pending_block() {}

pub fn get_latest_block() {}

