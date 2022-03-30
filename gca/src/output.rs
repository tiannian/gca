use alloc::vec::Vec;

use crate::{Amount, OutputId, OutputOperation};

pub enum OutputData {
    NativeToken(Amount),
    Data(Vec<u8>),
}

pub struct Output {
    pub data: OutputData,
    pub locker: OutputId,
    pub verifier: Option<OutputId>,
    pub operation: OutputOperation,
}
