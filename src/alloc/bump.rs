use core::sync::atomic::{AtomicPtr, Ordering};
use core::alloc::{GlobalAlloc, Layout};
use crate::util::{Buffer, Aligned8};

pub struct BumpAllocator<const N: usize> {
    buf: Aligned8<Buffer<N>>,
    ptr: AtomicPtr<u8>,
}

impl<const N: usize> BumpAllocator<N> {
    pub const fn new() -> Self {
        Self {
            buf: Aligned8(Buffer::new()),
            ptr: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub unsafe fn buffer(&self) -> &Buffer<N> {
        &self.buf.0
    }

    fn check_init(&self) {
        if self.ptr.load(Ordering::Acquire) == core::ptr::null_mut() {
            self.ptr.store(self.get_top_ptr(), Ordering::Relaxed);
        }
    }

    fn get_top_ptr(&self) -> *mut u8 {
        unsafe { self.buf.0.as_mut_ptr() }
    }

    fn get_ptr(&self) -> *mut u8 {
        self.ptr.load(Ordering::Acquire)
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn used(&self) -> usize {
        self.check_init();

        self.get_ptr() as usize - self.get_top_ptr() as usize
    }

    pub fn left(&self) -> usize {
        self.size() - self.used()
    }

    pub fn grow(&self, layout: Layout) -> *mut u8 {
        self.check_init();

        if layout.size() > self.left() {
            panic!("Out of memory!");
        }

        let mut full_size = layout.size();

        // TODO: Test alignment handling
        if layout.size() % layout.align() != 0 {
            full_size += layout.align() - (layout.size() % layout.align());
        }

        let res = self.get_ptr();

        self.ptr.store((res as usize + full_size) as *mut u8, Ordering::Release);

        res
    }
}

unsafe impl<const N: usize> GlobalAlloc for BumpAllocator<N> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.grow(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // noop for BumpAllocator, memory never really freed
    }
}

#[macro_export]
macro_rules! static_bump_allocator {
    (global, $alloc:ident, $size:expr) => {
        #[global_allocator]
        static $alloc: $crate::alloc::BumpAllocator<$size> = $crate::alloc::BumpAllocator::new();
    };

    (global, pub $alloc:ident, $size:expr) => {
        #[global_allocator]
        pub static $alloc: $crate::alloc::BumpAllocator<$size> = $crate::alloc::BumpAllocator::new();
    };

    ($alloc:ident, $size:expr) => {
        static $alloc: $crate::alloc::BumpAllocator<$size> = $crate::alloc::BumpAllocator::new();
    };

    (pub $alloc:ident, $size:expr) => {
        pub static $alloc: $crate::alloc::BumpAllocator<$size> = $crate::alloc::BumpAllocator::new();
    };
}
