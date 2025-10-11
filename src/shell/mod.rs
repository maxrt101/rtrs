use core::str::SplitWhitespace;
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::tty::{Tty, TtyEvent};
use crate::log::console::CONSOLE_OBJECT_NAME;
use crate::{print, println, object_with, object_with_mut};

use crate::channel::SubscriberId;

pub struct Command {
    pub name: &'static str,
    pub help: &'static str,
    pub handler: fn(SplitWhitespace) -> i8
}

#[macro_export]
macro_rules! commands {
    ($($cmd:expr),* $(,)?) => {
        &[$($cmd),*]
    };
}

#[macro_export]
macro_rules! command {
    ($name:expr, $help:expr, $handler:expr) => {
        $crate::shell::Command { name: $name, help: $help, handler: $handler }
    };
}

pub struct Shell {
    input: heapless::String<32>,
    commands: &'static [Command],
    input_changed: AtomicBool,
    tty_event_subscriber_id: SubscriberId,
}

// TODO: Make async?
impl Shell {
    pub fn new(commands: &'static [Command]) -> Self {
        Self {
            commands,
            input: heapless::String::new(),
            input_changed: AtomicBool::new(true),
            tty_event_subscriber_id: object_with_mut!(CONSOLE_OBJECT_NAME, Tty, tty, tty.subscribe().unwrap()),
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
        // FIXME: <SplitWhitespace as Iter>::next uses considerable amount of space
        let mut tokens = self.input.split_whitespace();

        if let Some(name) = tokens.next() {
            if name == "help" {
                for cmd in self.commands {
                    println!("{} - {}", cmd.name, cmd.help);
                }
            } else {
                let mut result: Option<i8> = None;

                for cmd in self.commands {
                    if cmd.name == name {
                        result = Some((cmd.handler)(tokens));
                        break;
                    }
                }

                if matches!(result, None) {
                    println!("Unknown command: {}", name);
                }
            }
        }

        self.input.clear();
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        object_with_mut!(CONSOLE_OBJECT_NAME, Tty, tty, tty.unsubscribe(self.tty_event_subscriber_id));
    }
}
