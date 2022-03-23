use alloc::vec::Vec;

use crate::{OutputId, OutputOperation};

pub struct Output {
    pub data: Vec<u8>,
    pub locker: OutputId,
    pub verifier: OutputId,
    pub operation: OutputOperation,
}

