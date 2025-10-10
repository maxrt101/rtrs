use core::sync::atomic::{AtomicIsize, Ordering};

pub type BorrowLocation = Option<&'static core::panic::Location<'static>>;

pub struct BorrowCounter {
    borrow: AtomicIsize,

    #[cfg(feature = "track_borrows")]
    borrowed_at: core::cell::Cell<BorrowLocation>,
}

impl BorrowCounter {
    pub const fn new() -> Self {
        Self {
            borrow: AtomicIsize::new(0),

            #[cfg(feature = "track_borrows")]
            borrowed_at: core::cell::Cell::new(None),
        }
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub unsafe fn acquire_ref(&self) {
        let current = self.borrow.load(Ordering::Acquire);

        if current < 0 {
            #[cfg(feature = "track_borrows")]
            if let Some(location) = self.borrowed_at.get() {
                panic!("Can't acquire ref, while mutable ref already acquired at {}:{}", location.file(), location.line())
            } else {
                panic!("Can't acquire ref, while mutable ref already acquired")
            }

            #[cfg(not(feature = "track_borrows"))]
            panic!("Can't acquire ref, while mutable ref already acquired")
        }

        #[cfg(feature = "track_borrows")]
        self.borrowed_at.set(Some(core::panic::Location::caller()));

        self.borrow.store(current + 1, Ordering::SeqCst);
    }

    pub unsafe fn release_ref(&self) {
        self.borrow.store(self.borrow.load(Ordering::Acquire) - 1, Ordering::SeqCst);
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub unsafe fn acquire_mut(&self) {
        let current = self.borrow.load(Ordering::Acquire);

        if current > 0 {
            #[cfg(feature = "track_borrows")]
            if let Some(location) = self.borrowed_at.get() {
                panic!("Can't acquire mutable ref, with {} refs already acquired (last borrow occurred at {}:{})", current, location.file(), location.line());
            } else {
                panic!("Can't acquire mutable ref, with {} refs already acquired", current);
            }

            #[cfg(not(feature = "track_borrows"))]
            panic!("Can't acquire mutable ref, with {} refs already acquired", current);
        }

        if current < 0 {
            #[cfg(feature = "track_borrows")]
            if let Some(location) = self.borrowed_at.get() {
                panic!("Can't acquire mut, already acquired at {}:{}", location.file(), location.line())
            } else {
                panic!("Can't acquire mut, already acquired")
            }

            #[cfg(not(feature = "track_borrows"))]
            panic!("Can't acquire mut, already acquired")
        }

        #[cfg(feature = "track_borrows")]
        self.borrowed_at.set(Some(core::panic::Location::caller()));

        self.borrow.store(current - 1, Ordering::SeqCst);
    }

    pub unsafe fn release_mut(&self) {
        self.borrow.store(self.borrow.load(Ordering::Acquire) + 1, Ordering::SeqCst);
    }
    
    pub unsafe fn reset_borrows(&self) {
        self.borrow.store(0, Ordering::SeqCst);
    }

    #[inline]
    pub fn used(&self) -> bool {
        self.borrow.load(Ordering::SeqCst) != 0
    }

    #[cfg(feature = "track_borrows")]
    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub fn borrowed_at(&self) -> BorrowLocation {
        self.borrowed_at.get()
    }
}

