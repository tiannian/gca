#[cfg(feature = "bump")]
mod bump;

mod handler;

#[cfg(feature = "wee-alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "bump")]
#[global_allocator]
static mut ALLOC: bump::BumpAllocator = bump::BumpAllocator {};
