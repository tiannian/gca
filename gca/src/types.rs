use alloc::vec::Vec;
use primitive_types::H256;

pub struct Txhash(pub H256);

pub struct Memo(pub Vec<u8>);

pub struct OutputOperation(pub u32);

