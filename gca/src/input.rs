use alloc::{string::String, vec::Vec};

use crate::OutputId;

#[derive(Debug, Clone)]
pub enum InputOperation {
    Input(u32),
    Reference(String, u32),
}

impl Default for InputOperation {
    fn default() -> Self {
        InputOperation::Input(0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Input {
    pub output_id: OutputId,
    pub unlock: Vec<u8>,
    pub operation: InputOperation,
}

