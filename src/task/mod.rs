pub mod futures;
pub mod sched;
pub mod event;
pub mod this;
pub mod tcb;
pub mod ctx;
mod waker;
mod macros;

pub use futures::{YieldFuture, TimeoutFuture};
pub use tcb::{TaskState, TaskControlBlock};
pub use ctx::ExecutionContext;
pub use event::Event;

use crate::task::tcb::TaskFlags;
use crate::task_yield;

extern crate alloc;
use alloc::boxed::Box;

use core::task::{Context, Poll};
use core::marker::PhantomData;
use core::pin::Pin;

/*
TODO: Based on `global` implement output redirecting demo (logger::log -> global::get_task().stdout.write() ?)
TODO: Create demo on dynamic task spawning
*/

pub struct Task<'a, R = ()> {
    future: Box<dyn Future<Output = R> + 'a>,
    tcb:    TaskControlBlock,
    _a:     PhantomData<&'a ()>,
}

impl<'a, R> Task<'a, R> {
    pub fn new<F: Future<Output = R> + 'a>(f: F) -> Self {
        Self {
            future: Box::new(f),
            tcb:    TaskControlBlock::new(),
            _a:     PhantomData,
        }
    }

    pub fn pend(&self) {
        let _ = self.tcb.lock();
        self.tcb.pend();
    }

    pub fn ready(&self) {
        let _ = self.tcb.lock();
        self.tcb.ready()
    }

    pub fn done(&self) {
        let _ = self.tcb.lock();
        self.tcb.done()
    }

    pub fn prio(&self) -> u8 {
        self.tcb.prio()
    }

    pub fn set_prio(&mut self, prio: u8) {
        let _ = self.tcb.lock();
        self.tcb.set_prio(prio);
    }

    pub fn get_clear_flag(&self, flag: TaskFlags) -> bool {
        let _ = self.tcb.lock();
        self.tcb.get_clear_flag(flag)
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
        let _ = self.tcb.lock();

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

unsafe impl<'a, R> Send for Task<'a, R> {}
unsafe impl<'a, R> Sync for Task<'a, R> {}
