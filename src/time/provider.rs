use core::sync::atomic::{AtomicU32, Ordering};
use crate::object::Object;

pub const TIME_OBJECT_NAME: &str = "time";

#[repr(transparent)]
pub struct TimeProvider {
    pub counter: AtomicU32,
}

impl TimeProvider {
    pub const fn new() -> Self {
        Self { counter: AtomicU32::new(0) }
    }

    pub fn now(&self) -> u32 {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn increment(&self) {
        self.counter.store(self.now() + 1, Ordering::Relaxed);
    }
}

impl Object for TimeProvider {}
