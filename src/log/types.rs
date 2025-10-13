extern crate alloc;

#[cfg(feature = "log_meta_dynamic")]
pub type MetaStorage<K, V> = alloc::collections::BTreeMap<K, V>;

#[cfg(feature = "log_meta_fixed")]
const LOG_META_CAPACITY: usize = envint::envint!("RTRS_LOG_META_FIXED_SIZE", 16);

#[cfg(feature = "log_meta_fixed")]
pub type MetaStorage<K, V> = heapless::FnvIndexMap<K, V, LOG_META_CAPACITY>;
