use core::pin::Pin;
use core::task::{Context, Poll};

pub struct YieldFuture {
    cycles: u32,
    counter: u32,
}

impl YieldFuture {
    pub fn new(cycles: u32) -> Self {
        Self { cycles, counter: 0 }
    }
}

impl Future for YieldFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.counter >= self.cycles {
            Poll::Ready(())
        } else {
            self.counter += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! task_yield {
    () => {
        crate::task::YieldFuture::new(1).await
    };
    ($cycles:expr) => {
        crate::task::YieldFuture::new($cycles).await
    }
}


pub struct TimeoutFuture {
    ticks: u32,
    start: u32,
}

impl TimeoutFuture {
    pub fn new(ticks: u32) -> Self {
        Self { ticks, start: crate::time::global_tick() }
    }
}

impl Future for TimeoutFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if crate::time::global_tick() - self.start >= self.ticks {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! task_sleep {
    ($ticks:expr) => {
        crate::task::TimeoutFuture::new($ticks).await
    };
}
