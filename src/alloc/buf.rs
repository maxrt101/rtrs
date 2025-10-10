use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::util::{Buffer, Aligned8};

pub struct BufferAllocator<const N: usize> {
    buf: Aligned8<Buffer<N>>,
    used: AtomicBool
}

impl<const N: usize> BufferAllocator<N> {
    pub const fn new() -> BufferAllocator<N> {
        BufferAllocator {
            buf: Aligned8(Buffer::new()),
            used: AtomicBool::new(false)
        }
    }

    pub unsafe fn buffer(&self) -> &Buffer<N> {
        &self.buf.0
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn used(&self) -> bool {
        self.used.load(Ordering::Relaxed)
    }

    pub fn allocate(&self, layout: Layout) -> *mut u8  {
        if self.used() {
            panic!("Already in use");
        }
        
        self.used.store(true, Ordering::Release);
        
        let mut ptr = unsafe { self.buf.0.as_mut_ptr() };

        // TODO: consider alignment
        if layout.size() > self.buf.0.size() {
            panic!("Not enough memory");
        }

        let align = ptr as usize % layout.align();

        if align != 0 {
            ptr = (ptr as usize + align) as *mut u8;
        }

        ptr
    }

    pub fn free(&self, _ptr: *mut u8) {
        self.used.store(false, Ordering::Release);
    }
}

unsafe impl<const N: usize> GlobalAlloc for BufferAllocator<N> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.free(ptr);
    }
}

#[macro_export]
macro_rules! static_buffer_allocator {
    ($alloc:ident, $size:expr) => {
        static $alloc: $crate::alloc::BufferAllocator<$size> = $crate::alloc::BufferAllocator::new();
    };

    (pub $alloc:ident, $size:expr) => {
        static $alloc: $crate::alloc::BufferAllocator<$size> = $crate::alloc::BufferAllocator::new();
    };

    (global, $alloc:ident, $size:expr) => {
        #[global_allocator]
        static $alloc: $crate::alloc::BufferAllocator<$size> = $crate::alloc::BufferAllocator::new();
    };

    (global, pub $alloc:ident, $size:expr) => {
        #[global_allocator]
        static $alloc: $crate::alloc::BufferAllocator<$size> = $crate::alloc::BufferAllocator::new();
    };
}
