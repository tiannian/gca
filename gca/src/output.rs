use alloc::vec::Vec;

use crate::{Amount, OutputId, OutputOperation};

#[derive(Debug)]
pub enum OutputData {
    NativeToken(Amount),
    Data(Vec<u8>),
}

impl Default for OutputData {
    fn default() -> Self {
        Self::NativeToken(Amount::default())
    }
}

#[derive(Debug, Default)]
pub struct OutputCore {
    pub data: OutputData,
    pub locker: OutputId,
    pub verifier: Option<OutputId>,
}

#[derive(Debug, Default)]
pub struct Output {
    pub core: OutputCore,
    pub operation: OutputOperation,
}
