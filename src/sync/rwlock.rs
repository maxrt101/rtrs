use core::cell::UnsafeCell;

use crate::sync::{BorrowCounter, BorrowLocation, Ref, RefMut};

pub struct RwLock<T> {
    data: UnsafeCell<Option<T>>,
    borrow: BorrowCounter,
}

impl<T> RwLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(Some(data)),
            borrow: BorrowCounter::new(),
        }
    }

    pub const fn uninit() -> Self {
        Self {
            data: UnsafeCell::new(None),
            borrow: BorrowCounter::new(),
        }
    }

    pub fn invalidate(&self) {
        unsafe { *self.data.get() = None; }
    }

    pub unsafe fn get(&self) -> &mut T {
        unsafe { &mut *self.data.get() }.as_mut().unwrap()
    }
    
    pub unsafe fn reset_borrows(&self) {
        unsafe { self.borrow.reset_borrows(); }
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub fn lock(&self) -> Ref<'_, T> {
        unsafe { self.borrow.acquire_ref() };

        Ref::new(unsafe { self.get() }, &self.borrow)
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub fn lock_mut(&self) -> RefMut<'_, T> {
        unsafe { self.borrow.acquire_mut() };

        RefMut::new(unsafe { self.get() }, &self.borrow)
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub fn with<U>(&self, f: &impl Fn(&T) -> U) -> U {
        let lock = self.lock();

        f(&*lock)
    }

    #[cfg_attr(feature = "track_borrows", track_caller)]
    pub fn with_mut<U>(&self, f: &impl Fn(&mut T) -> U) -> U {
        let mut lock = self.lock_mut();

        f(&mut *lock)
    }

    pub fn take(self) -> T {
        self.data.into_inner().take().unwrap()
    }

    pub fn replace(&self, data: T) {
        if self.used() {
            #[cfg(feature = "track_borrows")]
            rwlock_report_replace_used(self.borrow.borrowed_at());

            #[cfg(not(feature = "track_borrows"))]
            rwlock_report_replace_used();

            // #[cfg(feature = "track_borrows")]
            // if let Some(location) = self.borrow.borrowed_at() {
            //     panic!("Can't replace, value is used (last borrow at {}:{})", location.file(), location.line())
            // } else {
            //     panic!("Can't replace, value is used")
            // }
            //
            // #[cfg(not(feature = "track_borrows"))]
            // panic!("Can't replace, value is used")
        }

        unsafe { *self.data.get() = Some(data); }
    }

    pub fn take_map<U>(self, f: impl FnOnce(T) -> U) -> U {
        f(self.data.into_inner().take().unwrap())
    }

    #[inline]
    pub fn used(&self) -> bool {
        self.borrow.used()
    }

    #[inline]
    pub fn has_value(&self) -> bool {
        matches!(unsafe { &*self.data.get() }, Some(_))
    }
}

#[cfg(feature = "track_borrows")]
fn rwlock_report_replace_used(location: BorrowLocation) {
    if let Some(location) = location {
        panic!("Can't replace, value is used (last borrow at {}:{})", location.file(), location.line())
    } else {
        panic!("Can't replace, value is used")
    }
}

#[cfg(not(feature = "track_borrows"))]
fn rwlock_report_replace_used() {
    panic!("Can't replace, value is used")
}

unsafe impl<T> Sync for RwLock<T> where T: Send {}
unsafe impl<T> Send for RwLock<T> where T: Send {}
