use alloc::string::String;
use cstr_core::{c_char, CStr};

use crate::Result;

#[link(wasm_import_module = "_gca_env")]
extern "C" {
    fn _gca_env_get_chain_id() -> *const c_char;
}

pub fn get_chain_id() -> Result<String> {
    let s = unsafe {
        let ptr = _gca_env_get_chain_id();

        CStr::from_ptr(ptr)
    };

    Ok(String::from(s.to_str()?))
}
