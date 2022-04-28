use alloc::vec::Vec;

use crate::{Amount, OutputId, OutputOperation};

#[derive(Debug, Clone)]
pub enum OutputData {
    NativeToken(Amount),
    Data(Vec<u8>),
}

impl Default for OutputData {
    fn default() -> Self {
        Self::NativeToken(Amount::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct OutputCore {
    pub data: OutputData,
    pub locker: OutputId,
    pub verifier: Option<OutputId>,
    pub owner: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct Output {
    pub core: OutputCore,
    pub operation: OutputOperation,
}
