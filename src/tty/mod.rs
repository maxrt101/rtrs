pub mod ansi;
pub mod ascii;

use crate::itc::channel::{Channel, SubscriberId};
use crate::object::Object;
use crate::time::Timeout;

pub enum TtyFlag {
    EchoOutput = 1 << 0,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TtyEvent {
    None          = 0,
    WriteHappened = 1 << 0,
}

pub struct Tty {
    read:   fn() -> Option<u8>,
    write:  fn(u8),
    flags:  u8,
    // pubsub: Publisher<TtyEvent>,
    event_channel: Channel<TtyEvent>,
}

impl Tty {
    pub fn new(write: fn(u8), read: fn() -> Option<u8>) -> Self {
        Self { write, read, flags: 0, event_channel: Channel::<TtyEvent>::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.event_channel.send(TtyEvent::WriteHappened);
        (self.write)(byte);
    }

    pub fn read(&self) -> Option<u8> {
        (self.read)()
    }

    pub fn read_blocking(&self, timeout: Timeout) -> Option<u8> {
        while !timeout.expired() {
            let res = self.read();

            if res.is_some() {
                return res;
            }
        }

        None
    }

    pub fn subscribe(&mut self) -> Option<SubscriberId> {
        self.event_channel.subscribe()
    }

    pub fn unsubscribe(&mut self, id: SubscriberId) {
        self.event_channel.unsubscribe(id)
    }

    pub fn recv_event(&mut self, id: SubscriberId) -> Option<TtyEvent> {
        self.event_channel.recv(id)
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
