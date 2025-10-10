
pub mod heap;
pub mod bump;
pub mod buf;

pub use heap::HeapAllocator;
pub use bump::BumpAllocator;
pub use buf::BufferAllocator;

pub trait Allocator {
    unsafe fn alloc(layout: core::alloc::Layout) -> *mut u8;
    unsafe fn dealloc(ptr: *mut u8);
}
