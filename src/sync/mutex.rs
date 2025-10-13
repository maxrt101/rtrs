use core::sync::atomic::{self, AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use crate::sync::BorrowLocation;

pub struct MutexGuard<'a, T, L> {
    data: &'a T,
    lock: &'a Mutex<L>,
}

impl<'a, T, L> MutexGuard<'a, T, L> {
    pub fn map<U>(&mut self, f: fn(&T) -> &U) -> &U {
        f(self.data)
    }

    pub fn map_into<U>(self, f: impl FnOnce(&T) -> &U) -> MutexGuard<'a, U, L> {
        let this = ManuallyDrop::new(self);

        MutexGuard {
            data: unsafe { &*(f(this.data) as * const U) },
            lock: this.lock,
        }
    }
}

impl<'a, L> MutexGuard<'a, L, L> {
    fn from_mutex(lock: &'a Mutex<L>) -> Self {
        Self { data: unsafe { lock.get() }, lock }
    }
}

impl<'a, T, L> Drop for MutexGuard<'a, T, L> {
    fn drop(&mut self) {
        unsafe { self.lock.release() }
    }
}

impl<'a, T, L> Deref for MutexGuard<'a, T, L> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


pub struct MutexGuardMut<'a, T, L> {
    data: &'a mut T,
    lock: &'a Mutex<L>,
}

impl<'a, T, L> MutexGuardMut<'a, T, L> {
    pub fn map<U>(&mut self, f: fn(&T) -> &U) -> &U {
        f(self.data)
    }

    pub fn map_into<U>(self, f: impl FnOnce(&mut T) -> &mut U) -> MutexGuardMut<'a, U, L> {
        let mut this = ManuallyDrop::new(self);

        MutexGuardMut {
            data: unsafe { &mut *(f(this.data.deref_mut()) as * mut U) },
            lock: this.lock,
        }
    }
}

impl<'a, L> MutexGuardMut<'a, L, L> {
    fn from_mutex(lock: &'a Mutex<L>) -> Self {
        Self { data: unsafe { lock.get() }, lock }
    }
}

impl<'a, T, L> Drop for MutexGuardMut<'a, T, L> {
    fn drop(&mut self) {
        unsafe { self.lock.release() }
    }
}

impl<'a, T, L> Deref for MutexGuardMut<'a, T, L> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T, L> DerefMut for MutexGuardMut<'a, T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}


#[derive(Eq, PartialEq)]
pub enum RaceAction {
    Spin,
    Crash
}


pub struct Mutex<T> {
    data: UnsafeCell<Option<T>>,
    lock: AtomicBool,
    action: RaceAction,

    #[cfg(feature = "track_locks")]
    last_lock_location: core::cell::Cell<Option<&'static core::panic::Location<'static>>>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T, action: RaceAction) -> Self {
        Self {
            data: UnsafeCell::new(Some(data)),
            lock: AtomicBool::new(false),
            action,

            #[cfg(feature = "track_locks")]
            last_lock_location: core::cell::Cell::new(None),
        }
    }

    pub const fn uninit(action: RaceAction) -> Self {
        Self {
            data: UnsafeCell::new(None),
            lock: AtomicBool::new(false),
            action,

            #[cfg(feature = "track_locks")]
            last_lock_location: core::cell::Cell::new(None),
        }
    }

    pub fn invalidate(&self) {
        unsafe { *self.data.get() = None; }
    }

    pub unsafe fn acquire(&self) {
        if self.action == RaceAction::Spin {
            while self.lock.load(Ordering::Relaxed) {
                atomic::compiler_fence(Ordering::SeqCst);
            }
        } else {
            if self.lock.load(Ordering::Relaxed) {
                #[cfg(feature = "track_locks")]
                mutex_acquire_report_held(self.last_lock_location.get());

                #[cfg(not(feature = "track_locks"))]
                mutex_acquire_report_held();
            }
        }

        self.lock.store(true, Ordering::Relaxed);

        #[cfg(feature = "critical_section_custom")]
        unsafe { crate::sync::rtrs_critical_section_acquire() };
    }

    pub unsafe fn release(&self) {
        self.lock.store(false, Ordering::Relaxed);

        #[cfg(feature = "critical_section_custom")]
        unsafe { crate::sync::rtrs_critical_section_release() };
    }

    fn try_acquire(&self) -> bool {
        if self.lock.load(Ordering::Relaxed) {
            false
        } else {
            self.lock.store(true, Ordering::Relaxed);
            true
        }
    }

    pub unsafe fn get(&self) -> &mut T {
        unsafe { &mut *self.data.get() }.as_mut().unwrap()
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn lock(&self) -> MutexGuard<'_, T, T> {
        unsafe { self.acquire(); }

        #[cfg(feature = "track_locks")]
        self.last_lock_location.set(Some(core::panic::Location::caller()));

        MutexGuard::from_mutex(self)
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn lock_mut(&self) -> MutexGuardMut<'_, T, T> {
        unsafe { self.acquire(); }

        #[cfg(feature = "track_locks")]
        self.last_lock_location.set(Some(core::panic::Location::caller()));

        MutexGuardMut::from_mutex(self)
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T, T>> {
        if self.try_acquire() {
            #[cfg(feature = "track_locks")]
            self.last_lock_location.set(Some(core::panic::Location::caller()));

            Some(MutexGuard::from_mutex(self))
        } else {
            None
        }
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn try_lock_mut(&self) -> Option<MutexGuardMut<'_, T, T>> {
        if self.try_acquire() {
            #[cfg(feature = "track_locks")]
            self.last_lock_location.set(Some(core::panic::Location::caller()));

            Some(MutexGuardMut::from_mutex(self))
        } else {
            None
        }
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn with<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        let lock = self.lock();

        #[cfg(feature = "track_locks")]
        self.last_lock_location.set(Some(core::panic::Location::caller()));

        f(&*lock)
    }

    #[cfg_attr(feature = "track_locks", track_caller)]
    pub fn with_mut<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        let mut lock = self.lock_mut();

        #[cfg(feature = "track_locks")]
        self.last_lock_location.set(Some(core::panic::Location::caller()));

        f(&mut *lock)
    }

    pub fn take(self) -> T {
        self.data.into_inner().take().unwrap()
    }

    pub fn take_map<U>(self, f: impl FnOnce(T) -> U) -> U {
        f(self.data.into_inner().take().unwrap())
    }
    
    pub fn replace(&self, data: T) {
        struct EmptyMutexGuard<'a, T> {
            lock: &'a Mutex<T>
        }
        
        impl<'a, T> Drop for EmptyMutexGuard<'a, T> {
            fn drop(&mut self) {
                unsafe { self.lock.release() }
            }
        }
    
        unsafe { self.acquire(); }
        
        let _guard = EmptyMutexGuard { lock: self };
        
        unsafe { *self.data.get() = Some(data); }
    }

    #[inline]
    pub fn locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn has_value(&self) -> bool {
        matches!(unsafe { &*self.data.get() }, Some(_))
    }
}

#[cfg(feature = "track_locks")]
fn mutex_acquire_report_held(location: BorrowLocation) {
    if let Some(location) = location {
        panic!("Mutex is already held at {}:{}", location.file(), location.line());
    } else {
        panic!("Mutex is already held");
    }
}

#[cfg(not(feature = "track_locks"))]
fn mutex_acquire_report_held() {
    panic!("Mutex is already held");
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}
unsafe impl<T> Send for Mutex<T> where T: Send {}
