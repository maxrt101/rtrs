pub mod ansi;
pub mod ascii;

use crate::itc::channel::{Channel, SubscriberId};
use crate::object::Object;
use crate::time::Timeout;

extern crate alloc;
use alloc::boxed::Box;

pub trait TtyBackend {
    fn read(&mut self) -> Option<u8>;
    fn write(&mut self, byte: u8);
}

pub enum TtyFlag {
    EchoOutput = 1 << 0,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TtyEvent {
    None          = 0,
    WriteHappened = 1 << 0,
}

pub struct Tty {
    backend: Box<dyn TtyBackend + Send + Sync + 'static>,
    events:  Channel<TtyEvent>,
    flags:   u8,
}

impl Tty {
    pub fn new<T: TtyBackend + Send + Sync + 'static>(backend: T) -> Self {
        Self {
            backend: Box::new(backend),
            events: Channel::<TtyEvent>::new(),
            flags: 0,
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.events.send(TtyEvent::WriteHappened);
        (*self.backend).write(byte);
    }

    pub fn read(&mut self) -> Option<u8> {
        (*self.backend).read()
    }

    pub fn read_blocking(&mut self, timeout: Timeout) -> Option<u8> {
        while !timeout.expired() {
            let res = self.read();

            if res.is_some() {
                return res;
            }
        }

        None
    }

    pub fn subscribe(&mut self) -> Option<SubscriberId> {
        self.events.subscribe()
    }

    pub fn unsubscribe(&mut self, id: SubscriberId) {
        self.events.unsubscribe(id)
    }

    pub fn recv_event(&mut self, id: SubscriberId) -> Option<TtyEvent> {
        self.events.recv(id)
    }

    pub fn set_flag(&mut self, flag: TtyFlag, value: bool) {
        if value {
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }
}

impl Object for Tty {}
