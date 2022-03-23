use alloc::vec::Vec;

use crate::Txhash;

pub struct OutputId {
    pub txhash: Txhash,
    pub n: u64,
}

pub enum InputOperation {
    Input(u32),
    Reference,
}

pub struct Input {
    pub output_id: OutputId,
    pub unlock: Vec<u8>,
    pub operation: InputOperation,
}

