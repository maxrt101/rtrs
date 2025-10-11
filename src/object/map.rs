extern crate alloc;

#[cfg(feature = "storage_dynamic")]
pub type Map<K, V> = alloc::collections::BTreeMap<K, V>;

#[cfg(feature = "storage_fixed_16")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 16>;

#[cfg(feature = "storage_fixed_64")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 64>;

#[cfg(feature = "storage_fixed_128")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 128>;