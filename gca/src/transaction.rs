use alloc::vec::Vec;

use crate::{Txhash, Input, Output, Memo};

pub struct Transaction {
    pub txhash: Txhash,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub memos: Vec<Memo>,
}
