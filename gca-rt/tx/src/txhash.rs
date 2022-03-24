use primitive_types::H256;

extern "C" {
    fn _get_txhash(txhash_ptr: *mut u8);
}

pub fn get_txhash() -> H256 {
    let mut txhash = H256::default();

    let txhash_ref = &mut txhash.0;

    unsafe {
        _get_txhash(txhash_ref.as_mut_ptr());
    }

    txhash
}
