#![no_std]
#![no_main]

extern crate gca_rt_panic_log;

#[no_mangle]
extern "C" fn _gca_unlock_entry() {
    main()
}

pub fn main() {}
