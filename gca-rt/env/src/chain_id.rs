use alloc::string::String;
use cstr_core::CStr;

use crate::Result;

extern "C" {
    fn _env_get_chain_id() -> *const i8;
}

pub fn get_chain_id() -> Result<String> {
    let s = unsafe {
        let ptr = _env_get_chain_id();

        CStr::from_ptr(ptr)
    };

    Ok(String::from(s.to_str()?))
}
