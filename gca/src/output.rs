use alloc::vec::Vec;

use crate::{OutputId, OutputOperation, AssetType, Amount};

pub enum OutputData {
    Token(AssetType, Amount),
    Data(Vec<u8>),
}

pub struct Output {
    pub data: OutputData,
    pub locker: OutputId,
    pub verifier: OutputId,
    pub operation: OutputOperation,
}

