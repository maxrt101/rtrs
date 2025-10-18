#[cfg(feature = "channel_dynamic")]
pub type ChannelVec = alloc::vec::Vec<T>;

#[cfg(feature = "channel_fixed")]
const CHANNEL_SIZE: usize = envint::envint!("RTRS_CHANNEL_SIZE", 32);

#[cfg(feature = "channel_fixed")]
pub type ChannelVec<T> = heapless::Vec<T, CHANNEL_SIZE>;
