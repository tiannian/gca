use crate::{Result, Error};

extern "C" {
    fn _input_get_count() -> usize;

    fn _input_get_type_by_index(idx: usize) -> u32;

    fn _input_get_output_id_by_index(idx: usize, txhash_ptr: *mut u8) -> u64;

    fn _input_get_unlock_data_by_index(idx: usize, unlock_data_ptr: *mut u8) -> usize;
}

pub enum InputType {
    Transfer
}

impl InputType {
    fn from_u32(t: u32) -> Result<Self> {
        match t {
            1 => Ok(Self::Transfer),
            _ => Err(Error::InputTypeIndexError)
        }
    }
}

pub fn get_type(idx: usize) -> Result<InputType> {
    let t = unsafe {
        _input_get_type_by_index(idx)
    };

    InputType::from_u32(t)
}

// pub fn get_output_id(idx: usize) -> Result<>

