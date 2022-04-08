#![no_std]
#![no_main]

extern crate gca_rt_panic_log;

#[no_mangle]
extern "C" fn _gca_unlock_entry() {
    main()
}

#[no_mangle]
extern "C" fn _gca_operation_entry() {
    main()
}

#[no_mangle]
extern "C" fn _gca_verifier_entry() {
    main()
}

pub fn main() {}
