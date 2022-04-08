#![no_std]
#![feature(alloc_error_handler, core_intrinsics)]

extern crate alloc;

mod mem;

mod chain_id;
pub use chain_id::*;

mod error;
pub use error::*;

#[cfg(target_arch = "wasm32")]
mod allocator;
#[cfg(target_arch = "wasm32")]
pub use allocator::*;
