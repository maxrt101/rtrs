use core::str::SplitWhitespace;
use core::fmt::Write;

use crate::tty::{Tty, TtyStateFlag};
use crate::log::console::CONSOLE_OBJECT_NAME;
use crate::{print, println, object_with, object_with_mut};

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

pub struct Shell<'a> {
    input: heapless::String<32>,
    input_changed: bool,
    commands: &'a [Command]
}

// TODO: Make async?
impl<'a> Shell<'a> {
    pub fn new(commands: &'a [Command]) -> Self {
        Self {
            input: heapless::String::new(),
            input_changed: true,
            commands
        }
    }

    pub fn cycle(&mut self) {
        self.prompt();

        let byte = object_with!(CONSOLE_OBJECT_NAME, Tty, console, {
            console.read_byte()
        });

        if let Some(b) = byte {
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

            self.input_changed = true;
        }
    }

    fn prompt(&mut self) {
        let write_happened = object_with_mut!(CONSOLE_OBJECT_NAME, Tty, console, {
            console.get_state_flag(TtyStateFlag::WriteHappened)
        });

        if self.input_changed || write_happened {
            print!("{}\r# {}", crate::ANSI_ERASE_FROM_CURSOR_TO_LINE_START, self.input);

            object_with_mut!(CONSOLE_OBJECT_NAME, Tty, console, {
                console.set_state_flag(TtyStateFlag::WriteHappened, false)
            });

            self.input_changed = false;
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
