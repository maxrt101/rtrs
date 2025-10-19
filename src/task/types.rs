#[cfg(feature = "sched_storage_dynamic")]
pub type SchedulerStorage<T> = alloc::vec::Vec<T>;

#[cfg(feature = "sched_storage_fixed")]
const SCHED_STORAGE_SIZE: usize = envint::envint!("RTRS_SCHED_STORAGE_SIZE", 8);

#[cfg(feature = "sched_storage_fixed")]
pub type SchedulerStorage<T> = heapless::Vec<T, SCHED_STORAGE_SIZE>;
