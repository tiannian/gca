use core::{alloc::Layout, ptr::null_mut};

#[no_mangle]
extern "C" fn _gca_env_alloc(size: usize) -> *mut u8 {
    if let Ok(layout) = Layout::array::<u8>(size) {
        unsafe { alloc::alloc::alloc(layout) }
    } else {
        null_mut()
    }
}

#[no_mangle]
extern "C" fn _gca_env_free(ptr: *mut u8, size: usize) {
    if let Ok(layout) = Layout::array::<u8>(size) {
        unsafe {
            alloc::alloc::dealloc(ptr, layout)
        }
    }
}

