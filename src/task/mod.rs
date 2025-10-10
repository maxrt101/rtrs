pub mod futures;
pub mod event;
pub mod tcb;
pub mod ctx;
mod waker;
mod macros;

pub use ctx::ExecutionContext;
pub use tcb::{TaskState, TaskControlBlock};
pub use event::Event;
pub use futures::{YieldFuture, TimeoutFuture};
use crate::task_yield;

extern crate alloc;
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Task<'a, R = ()> {
    pub future: Box<dyn Future<Output = R> + 'a>,
    pub tcb: TaskControlBlock,
    _a: PhantomData<&'a ()>,
}

impl<'a, R> Task<'a, R> {
    pub fn new<F: Future<Output = R> + 'a>(f: F) -> Self {
        Self {
            future: Box::new(f),
            tcb: TaskControlBlock::new(),
            _a: PhantomData,
        }
    }

    #[inline]
    pub fn pend(&self) {
        self.tcb.pend();
    }

    #[inline]
    pub fn ready(&self) {
        self.tcb.ready()
    }

    #[inline]
    pub fn done(&self) {
        self.tcb.done()
    }

    #[inline]
    pub fn get_state(&self) -> TaskState {
        self.tcb.get_state()
    }

    #[inline]
    pub fn is_state(&self, s: TaskState) -> bool {
        self.tcb.is_state(s)
    }

    pub fn is_running(&self) -> bool {
        self.tcb.is_state(TaskState::Ready) || self.tcb.is_state(TaskState::Pending)
    }

    pub fn poll(&mut self) -> Poll<R> {
        if self.is_state(TaskState::Ready) {
            self.pend();

            let waker = waker::create_waker(&self.tcb as * const _ as *const ());

            let mut ctx = Context::from_waker(&waker);

            let pin = unsafe { Pin::new_unchecked(&mut *self.future) };

            let poll = pin.poll(&mut ctx);

            if poll.is_ready() {
                self.done();
            }

            poll
        } else {
            Poll::Pending
        }
    }

    pub async fn cycle(&mut self) {
        match self.poll() {
            Poll::Ready(_) => (),
            Poll::Pending => task_yield!(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.poll().is_ready() {
                break;
            }
        }
    }
}
