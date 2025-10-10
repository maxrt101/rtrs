use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::alloc::{Layout, GlobalAlloc};
use core::fmt::Write;
use crate::util::{Aligned8, Buffer};
use crate::{println};

#[repr(align(8))]
struct Block {
    size: AtomicUsize,
    used: AtomicBool,
    last: AtomicBool
}

impl Block {
    fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }

    fn used(&self) -> bool {
        self.used.load(Ordering::SeqCst)
    }

    fn last(&self) -> bool {
        self.last.load(Ordering::SeqCst)
    }

    fn ptr(&self) -> *mut u8 {
        self as *const _ as *mut _
    }

    fn data(&self) -> *mut u8 {
        let ptr = self.ptr();
        let ptr = ptr as usize + size_of::<Block>();
        ptr as *mut u8
    }

    fn next(&self) -> *mut Block {
        (self.data() as usize + self.size()) as *mut Block
    }

    fn use_unchecked(&self, size: usize) {
        let orig_size = self.size();

        self.size.store(size, Ordering::SeqCst);
        self.used.store(true, Ordering::SeqCst);

        // If there is no memory left to create new block for the rest of the memory
        if orig_size - size < size_of::<Block>() {
            self.size.store(orig_size, Ordering::SeqCst);
            return;
        }

        if size != orig_size {
            let next = unsafe { &*self.next() };

            next.used.store(false, Ordering::SeqCst);
            next.size.store(orig_size - size - size_of::<Block>(), Ordering::SeqCst);

            if self.last() {
                self.last.store(false, Ordering::SeqCst);
                next.last.store(true, Ordering::SeqCst);
            }
        }
    }

    fn free(&self) {
        self.used.store(false, Ordering::SeqCst);
    }

    fn merge_unchecked(&self) {
        let next = unsafe { &*self.next() };

        if next.last() {
            self.last.store(true, Ordering::SeqCst);
        }

        self.size.store(self.size() + next.size() + size_of::<Block>(), Ordering::SeqCst);
    }

    fn from_data(data: *mut u8) -> *mut Block {
        (data as usize - size_of::<Block>()) as *mut Block
    }
}

// TODO: Passing N like this causes each HeapAllocator<N> to be a different
//       type, with different (albeit the same) methods
//       Consider storing *mut u8 and usize
pub struct HeapAllocator<const N: usize> {
    buf: Aligned8<Buffer<N>>,
    initialized: AtomicBool,
}

impl<const N: usize> HeapAllocator<N> {
    pub const fn new() -> Self {
        Self {
            buf: Aligned8(Buffer::new()),
            initialized: AtomicBool::new(false),
        }
    }

    pub unsafe fn buffer(&self) -> &Buffer<N> {
        &self.buf.0
    }

    fn root(&self) -> *mut Block {
        unsafe { self.buf.0.as_mut_ptr() as *mut Block }
    }

    #[inline(never)]
    fn check_init(&self) {
        if !self.initialized.load(Ordering::SeqCst) {
            let root_blk = self.root();
            let root_block = unsafe { &mut *root_blk };

            root_block.size.store(N - size_of::<Block>(), Ordering::SeqCst);
            root_block.used.store(false, Ordering::SeqCst);
            root_block.last.store(true, Ordering::SeqCst);

            self.initialized.store(true, Ordering::SeqCst);
        }
    }

    fn defragment(&self) {
        self.check_init();

        let mut blk = self.root();

        loop {
            let block = unsafe { &mut *blk };
            let next = unsafe { &mut *block.next() };

            if block.last() {
                return;
            } else {
                if !block.used() && !next.used() {
                    block.merge_unchecked();
                } else {
                    blk = block.next();
                }
            }
        }
    }

    pub fn allocate(&self, layout: Layout) -> *mut u8 {
        self.check_init();

        let mut full_size = layout.size();

        if layout.size() % layout.align() != 0 {
            full_size += layout.align() - (layout.size() % layout.align());
        }

        let mut blk = self.root();

        loop {
            let block = unsafe { &mut *blk };

            if !block.used() && block.size() >= full_size {
                block.use_unchecked(full_size);

                return block.data();
            }

            if block.last() {
                // TODO: Or return null
                panic!("Out of memory");
            } else {
                blk = block.next();
            }
        }
    }

    pub fn free(&self, ptr: *mut u8) {
        if !self.initialized.load(Ordering::SeqCst) {
            panic!("Can't free on uninitialized allocator");
        }

        let bounds = unsafe { self.buf.0.mut_ptr_bounds() };

        if ptr < bounds.0 || ptr > bounds.1 {
            panic!("{:?} doesn't belong to this allocator", ptr);
        }

        let blk = Block::from_data(ptr);

        let block = unsafe { &*blk };

        if !block.used() {
            panic!("{:?} is invalid", ptr);
        }

        block.free();

        // TODO: Add config option for autodefrag after free
        self.defragment();
    }

    pub fn dump(&self) {
        self.check_init();

        let mut blk = self.root();

        loop {
            let block = unsafe { &mut *blk };

            println!("BLOCK {:?} used={} last={} data={:?} size={}", blk, block.used(), block.last(), block.data(), block.size());

            if block.last() {
                return;
            } else {
                blk = block.next();
            }
        }
    }
}

unsafe impl<const N: usize> GlobalAlloc for HeapAllocator<N> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.free(ptr)
    }
}

#[macro_export]
macro_rules! heap_allocator {
    ($alloc:ident, $size:expr) => {
        static $alloc: $crate::alloc::HeapAllocator<$size> = $crate::alloc::HeapAllocator::new();
    };

    (pub $alloc:ident, $size:expr) => {
        pub static $alloc: $crate::alloc::HeapAllocator<$size> = $crate::alloc::HeapAllocator::new();
    };

    (global, $alloc:ident, $size:expr) => {
        #[global_allocator]
        static $alloc: $crate::alloc::HeapAllocator<$size> = $crate::alloc::HeapAllocator::new();
    };

    (global, pub $alloc:ident, $size:expr) => {
        #[global_allocator]
        pub static $alloc: $crate::alloc::HeapAllocator<$size> = $crate::alloc::HeapAllocator::new();
    };
}
