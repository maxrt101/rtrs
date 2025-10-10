use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, Waker};
use crate::sync::RwLock;

pub struct Event {
    flag:  AtomicBool,
    waker: RwLock<Waker>,
}

impl Event {
    pub const fn new() -> Self {
        Self {
            flag:  AtomicBool::new(false),
            waker: RwLock::uninit(),
        }
    }

    pub fn trigger(&self) {
        self.flag.store(true, Ordering::Release);

        // FIXME: Check for data races
        if self.waker.has_value() {
            self.waker.with(&|waker| {
                waker.wake_by_ref();
            })
        }
    }
    
    pub fn clear(&self) {
        self.flag.store(false, Ordering::Release);
        self.waker.invalidate();
    }
}

impl Future for &Event {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // FIXME: Check for data races
        self.waker.replace(cx.waker().clone());

        if self.flag.load(Ordering::Acquire) {
            self.flag.store(false, Ordering::Release);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
