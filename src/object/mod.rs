mod types;
mod storage;
mod macros;

extern crate alloc;
use alloc::boxed::Box;

use core::any::Any;

use crate::sync::RwLock;

pub use storage::Storage;

pub trait Object: Any {}

pub type DynObject = dyn Object + Send + Sync;

pub type BoxObject = Box<DynObject>;

pub static STORAGE: RwLock<Storage> = RwLock::new(Storage::new());

