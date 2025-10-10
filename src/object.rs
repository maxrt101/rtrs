use crate::sync::{RwLock, Ref, RefMut};

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use core::any::Any;


pub trait Object: Any {}

pub type DynObject = dyn Object + Send + Sync;

pub type BoxObject = Box<DynObject>;

pub struct Storage {
    // storage: BTreeMap<&'static str, RwLock<BoxObject>>
    storage: heapless::FnvIndexMap<&'static str, RwLock<BoxObject>, 16>
}

impl Storage {
    pub const fn new() -> Self {
        Self {
            // storage: BTreeMap::new()
            storage: heapless::FnvIndexMap::new()
        }
    }

    pub fn get_raw(&'_ self, key: &str) -> Option<Ref<'_, BoxObject>> {
        match self.storage.get(key) {
            Some(lock) => Some(lock.lock()),
            None => None
        }
    }

    pub fn get_raw_mut(&'_ self, key: &str) -> Option<RefMut<'_, BoxObject>> {
        match self.storage.get(key) {
            Some(lock) => Some(lock.lock_mut()),
            None => None
        }
    }

    pub fn get<T: Object>(&'_ self, key: &str) -> Option<Ref<'_, T>> {
        match self.storage.get(key) {
            Some(lock) => Some(lock.lock()
                .map_into(|object| {
                    let r: &dyn Any = &**object;
                    r.downcast_ref::<T>().unwrap()
                })
            ),
            None => None
        }
    }

    pub fn get_mut<T: Object>(&'_ self, key: &str) -> Option<RefMut<'_, T>> {
        match self.storage.get(key) {
            Some(lock) => Some(lock.lock_mut()
                .map_into(|object| {
                    let r: &mut dyn Any = &mut **object;
                    r.downcast_mut::<T>().unwrap()
                })
            ),
            None => None
        }
    }

    pub fn insert<T: Object + Send + Sync>(&mut self, key: &'static str, object: T) {
        let _ = self.storage.insert(key, RwLock::new(Box::new(object)));
    }

    pub fn remove(&mut self, key: &str) -> Option<BoxObject> {
        match self.storage.remove(key) {
            Some(lock) => Some(lock.take()),
            None => None
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.storage.keys().into_iter().map(|s| *s)
    }

    pub unsafe fn unlock(&self, key: &str) {
        match self.storage.get(key) {
            Some(lock) => unsafe { lock.reset_borrows() },
            None => ()
        }
    }
}

pub static STORAGE: RwLock<Storage> = RwLock::new(Storage::new());


#[macro_export]
macro_rules! object_with {
    ($name:expr, $type:ty, $obj:ident, $blk:expr) => {{
        let storage = $crate::object::STORAGE.lock();
        let $obj = storage.get::<$type>($name).unwrap();
        $blk
    }};
}

#[macro_export]
macro_rules! object_with_mut {
    ($name:expr, $type:ty, $obj:ident, $blk:expr) => {{
        let storage = $crate::object::STORAGE.lock();
        let mut $obj = storage.get_mut::<$type>($name).unwrap();
        $blk
    }};
}

#[macro_export]
macro_rules! object_insert {
    ($name:expr, $value:expr) => {{
        let mut storage = $crate::object::STORAGE.lock_mut();
        storage.insert($name, $value);
    }};
}
