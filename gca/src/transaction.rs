use alloc::vec::Vec;

use crate::{Input, Memo, Output, Txhash};

pub struct Transaction {
    pub txhash: Txhash,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub memos: Vec<Memo>,
}
