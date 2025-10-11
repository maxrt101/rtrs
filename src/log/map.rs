extern crate alloc;

#[cfg(feature = "log_meta_dynamic")]
pub type Map<K, V> = alloc::collections::BTreeMap<K, V>;

#[cfg(feature = "log_meta_fixed_16")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 16>;

#[cfg(feature = "log_meta_fixed_64")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 64>;

#[cfg(feature = "log_meta_fixed_128")]
pub type Map<K, V> = heapless::FnvIndexMap<K, V, 128>;