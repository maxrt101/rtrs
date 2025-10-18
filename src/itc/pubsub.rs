extern crate alloc;
use alloc::vec::Vec;

use core::marker::PhantomData;

use crate::task_yield;

// TODO: Add + Default to everything (besides F?)
pub struct Subscriber<E: Copy + Clone, C: Copy + Clone = (), F: Fn(C, E) = fn(C, E)> {
    handler: F,
    context: C,
    _event: PhantomData<E>,
}

impl<E: Copy + Clone, C: Copy + Clone, F: Fn(C, E)> Subscriber<E, C, F> {
    pub fn new(handler: F, context: C) -> Self {
        Self { handler, context, _event: PhantomData }
    }
}

// TODO: Don't use Vec?
pub struct Publisher<E: Copy + Clone, C: Copy + Clone = (), F: Fn(C, E) = fn(C, E)> {
    subscribers: Vec<Subscriber<E, C, F>>,
}

impl<E: Copy + Clone, C: Copy + Clone, F: Fn(C, E)> Publisher<E, C, F> {
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self, subscriber: Subscriber<E, C, F>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&mut self, event: E) {
        for sub in &self.subscribers {
            (sub.handler)(sub.context, event);
        }
    }

    pub async fn publish_async(&mut self, event: E) {
        for sub in &self.subscribers {
            (sub.handler)(sub.context, event);
            task_yield!();
        }
    }
}
