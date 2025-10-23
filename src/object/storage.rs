use crate::object::{BoxObject, Object};
use crate::sync::{Ref, RefMut, RwLock};
use crate::ignore;

extern crate alloc;
use alloc::boxed::Box;

use core::any::Any;

pub struct Storage {
    storage: super::types::ObjectStorage<&'static str, RwLock<BoxObject>>
}

impl Storage {
    pub const fn new() -> Self {
        Self {
            storage: super::types::ObjectStorage::new()
        }
    }

    pub fn has_object(&self, name: &'static str) -> bool {
        self.storage.contains_key(name)
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

    fn acquire_ref<T: Object>(lock: &RwLock<BoxObject>) -> Ref<'_, T> {
        lock.lock()
            .map_into(|object| {
                let r: &dyn Any = &**object;
                r.downcast_ref::<T>().unwrap()
            })
    }

    fn acquire_ref_mut<T: Object>(lock: &RwLock<BoxObject>) -> RefMut<'_, T> {
        lock.lock_mut()
            .map_into(|object| {
                let r: &mut dyn Any = &mut **object;
                r.downcast_mut::<T>().unwrap()
            })
    }

    pub fn get<T: Object>(&'_ self, key: &str) -> Option<Ref<'_, T>> {
        match self.storage.get(key) {
            Some(lock) => Some(Self::acquire_ref(lock)),
            None => None
        }
    }

    pub fn get_mut<T: Object>(&'_ self, key: &str) -> Option<RefMut<'_, T>> {
        match self.storage.get(key) {
            Some(lock) => Some(Self::acquire_ref_mut(lock)),
            None => None
        }
    }

    pub fn try_get<T: Object>(&'_ self, key: &str) -> Option<Ref<'_, T>> {
        match self.storage.get(key) {
            Some(lock) =>
                if !lock.used() {
                    Some(Self::acquire_ref(lock))
                } else {
                    None
                }
            None => None
        }
    }

    pub fn try_get_mut<T: Object>(&'_ self, key: &str) -> Option<RefMut<'_, T>> {
        match self.storage.get(key) {
            Some(lock) =>
                if !lock.used() {
                    Some(Self::acquire_ref_mut(lock))
                } else {
                    None
                }
            None => None
        }
    }

    pub fn insert<T: Object + Send + Sync>(&mut self, key: &'static str, object: T) {
        ignore!(self.storage.insert(key, RwLock::new(Box::new(object))));
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
