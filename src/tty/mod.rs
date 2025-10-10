pub mod ansi;
pub mod ascii;

use crate::object::Object;

pub enum TtyFlag {
    EchoOutput = 1 << 0,
}

pub enum TtyStateFlag {
    WriteHappened = 1 << 0,
}

pub struct Tty {
    read_byte: fn() -> Option<u8>,
    write:     fn(&str),
    state:     u8,
    flags:     u8,
}

impl Tty {
    pub fn new(write: fn(&str), read_byte: fn() -> Option<u8>) -> Self {
        Self { write, read_byte, state: 0, flags: 0 }
    }

    pub fn write(&mut self, msg: &str) {
        self.set_state_flag(TtyStateFlag::WriteHappened, true);
        (self.write)(msg);
    }

    pub fn read_byte(&self) -> Option<u8> {
        (self.read_byte)()
    }

    pub fn get_flag(&self, flag: TtyFlag) -> bool {
        (self.flags & flag as u8) != 0
    }

    pub fn set_flag(&mut self, flag: TtyFlag, value: bool) {
        if value {
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }

    pub fn get_state_flag(&mut self, flag: TtyStateFlag) -> bool {
        (self.state & flag as u8) != 0
    }

    pub fn set_state_flag(&mut self, flag: TtyStateFlag, value: bool) {
        if value {
            self.state |= flag as u8;
        } else {
            self.state &= !(flag as u8);
        }
    }
}

impl Object for Tty {}
