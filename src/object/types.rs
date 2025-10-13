extern crate alloc;

#[cfg(feature = "storage_dynamic")]
pub type ObjectStorage<K, V> = alloc::collections::BTreeMap<K, V>;

#[cfg(feature = "storage_fixed")]
const STORAGE_SIZE: usize = envint::envint!("RTRS_STORAGE_FIXED_SIZE", 16);

#[cfg(feature = "storage_fixed")]
pub type ObjectStorage<K, V> = heapless::FnvIndexMap<K, V, STORAGE_SIZE>;
