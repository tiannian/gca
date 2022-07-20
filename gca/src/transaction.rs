use alloc::vec::Vec;

use crate::{Input, Memo, Output, Txhash};

#[derive(Debug, Default, Clone)]
pub struct Transaction {
    pub txhash: Txhash,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub memos: Vec<Memo>,
}

