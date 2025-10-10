use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use crate::sync::borrow::BorrowCounter;


pub struct Ref<'a, T> {
    data: &'a T,
    lock: &'a BorrowCounter,
}

impl<'a, T> Ref<'a, T> {
    pub fn new(data: &'a T, lock: &'a BorrowCounter) -> Self {
        Self { data, lock }
    }

    pub fn map<U>(&mut self, f: fn(&T) -> &U) -> &U {
        f(self.data)
    }

    pub fn map_into<U>(self, f: impl FnOnce(&T) -> &U) -> Ref<'a, U> {
        let this = ManuallyDrop::new(self);

        Ref::new(
            // FIXME: Why works here, but not in RefMut::map_into
            // unsafe { &*(f(this.data) as * const U) },
            f(this.data),
            this.lock
        )
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        unsafe { self.lock.release_ref(); }
    }
}

impl<'a, T> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        unsafe { self.lock.acquire_ref(); }
        Self { data: self.data, lock: self.lock }
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}


pub struct RefMut<'a, T> {
    data: &'a mut T,
    lock: &'a BorrowCounter,
}

impl<'a, T> RefMut<'a, T> {
    pub fn new(data: &'a mut T, lock: &'a BorrowCounter) -> Self {
        Self { data, lock }
    }

    pub fn map<U>(&mut self, f: fn(&T) -> &U) -> &U {
        f(self.data)
    }

    pub fn map_into<U>(self, f: impl FnOnce(&mut T) -> &mut U) -> RefMut<'a, U> {
        let mut this = ManuallyDrop::new(self);

        RefMut::new(
            // FIXME: Why doesn't work here, but works in Ref::map_info
            unsafe { &mut *(f(this.data) as * mut U) },
            // f(this.data),
            this.lock
        )
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        unsafe { self.lock.release_mut(); }
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

