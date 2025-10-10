use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct LazyTakeOnce<T> {
    data: UnsafeCell<Option<T>>,
    borrowed: AtomicBool,
}

impl<T> LazyTakeOnce<T> {
    pub const fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(Some(data)), borrowed: AtomicBool::new(false) }
    }

    pub const fn uninit() -> Self {
        Self { data: UnsafeCell::new(None), borrowed: AtomicBool::new(false) }
    }

    pub fn init(&self, data: T) {
        unsafe {
            *self.data.get() = Some(data);
        }
    }
    
    pub unsafe fn get(&self) -> *mut Option<T> {
        self.data.get()
    }

    pub fn take(&self) -> &T {
        if !self.borrowed.load(Ordering::Acquire) {
            self.borrowed.store(true, Ordering::Release);
            unsafe { &*self.data.get() }.as_ref().unwrap()
        } else {
            panic!("BorrowOnce can only borrow the value once");
        }
    }

    pub fn take_mut(&self) -> &mut T {
        if !self.borrowed.load(Ordering::Acquire) {
            self.borrowed.store(true, Ordering::Release);
            unsafe { &mut *self.data.get() }.as_mut().unwrap()
        } else {
            panic!("BorrowOnce can only borrow the value once");
        }
    }
}

unsafe impl<T: Send> Sync for LazyTakeOnce<T> {}