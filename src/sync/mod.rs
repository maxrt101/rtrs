pub mod mutex;
pub mod rwlock;
pub mod borrow;
pub mod refs;

pub use mutex::{Mutex, RaceAction};
pub use rwlock::RwLock;
pub use refs::{Ref, RefMut};
pub use borrow::{BorrowCounter, BorrowLocation};

#[cfg(feature = "lock_custom")]
unsafe extern "Rust" {
    fn rtrs_lock_acquire();
    fn rtrs_lock_release();
}
