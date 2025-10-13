pub mod script;
pub mod command;
mod types;

use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::tty::{Tty, TtyEvent};
use crate::channel::SubscriberId;
use crate::log::console::CONSOLE_OBJECT_NAME;
use crate::{print, println, object_with, object_with_mut};

use command::Command;

crate::logger!("SHELL");


#[macro_export]
macro_rules! shell {
    ($($cmd:expr),* $(,)?) => {
        rtrs::shell::Shell::new(&[$($cmd),*])
    };
}

pub struct Shell {
    env:                     script::Environment,
    // TODO: Make size configurable (same as in object::map::Map & log::map::Map)
    input:                   types::Input,
    input_changed:           AtomicBool,
    tty_event_subscriber_id: SubscriberId,
}

// TODO: Make async?
impl Shell {
    pub fn new(commands: &'static [Command]) -> Self {
        Self {
            env:           script::Environment::new(commands),
            input:         heapless::String::new(),
            input_changed: AtomicBool::new(true),
            tty_event_subscriber_id: {
                object_with_mut!(CONSOLE_OBJECT_NAME, Tty, tty, tty.subscribe().unwrap())
            },
        }
    }

    pub fn cycle(&mut self) {
        self.prompt();

        if let Some(b) = object_with!(CONSOLE_OBJECT_NAME, Tty, tty, tty.read()) {
            match b {
                crate::ASCII_KEY_CR | crate::ASCII_KEY_LF => { // ASCII Enter
                    println!();
                    self.run();
                }
                crate::ASCII_KEY_BS | crate::ASCII_KEY_DEL => { // ASCII Backspace/Delete
                    self.input.pop();
                }
                _ => {
                    self.input.push(b as char).unwrap();
                }
            }

            self.input_changed.store(true, Ordering::SeqCst);
        }
    }

    fn prompt(&mut self) {
        let tty_event = object_with_mut!(CONSOLE_OBJECT_NAME, Tty, console, {
            console.recv_event(self.tty_event_subscriber_id)
        });

        if self.input_changed.load(Ordering::Acquire) || matches!(tty_event, Some(TtyEvent::WriteHappened)) {
            print!("{}\r# {}", crate::ANSI_ERASE_FROM_CURSOR_TO_LINE_START, self.input);

            self.input_changed.store(false, Ordering::SeqCst);
        }
    }

    fn run(&mut self) {
        self.env.run(self.input.as_str());

        self.input.clear();
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        object_with_mut!(CONSOLE_OBJECT_NAME, Tty, tty, tty.unsubscribe(self.tty_event_subscriber_id));
    }
}
