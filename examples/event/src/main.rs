#![no_std]
#![no_main]

extern crate gca_rt_panic_log;
extern crate gca_rt_env;

#[no_mangle]
extern "C" fn _gca_unlock_entry(_ptr: *const u8) -> i32 {
    main()
}

#[no_mangle]
extern "C" fn _gca_operation_entry() -> i32 {
    main()
}

#[no_mangle]
extern "C" fn _gca_verifier_entry() -> i32 {
    main()
}

pub fn main() -> i32 {
    gca_rt_log::init().unwrap();
    gca_rt_event::emit("test", b"aaaa", b"bbb", true);
    0
}
