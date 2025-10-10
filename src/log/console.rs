use crate::util::traits::Empty;
use crate::object_with_mut;
use crate::tty::Tty;

pub const CONSOLE_OBJECT_NAME: &str = "console";

pub struct ConsoleWriter {}

impl ConsoleWriter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Empty for ConsoleWriter {
    fn empty() -> Self {
        ConsoleWriter::new()
    }
}

impl core::fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        object_with_mut!(CONSOLE_OBJECT_NAME, Tty, console, {
            for c in s.chars() {
                console.write(c as u8);
            }
        });

        Ok(())
    }
}

#[macro_export]
macro_rules! raw_print {
    ($($arg:tt)*) => {{
        let mut console = $crate::log::console::ConsoleWriter::new();
        write!(&mut console, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! print {
    ($fmt:expr $(, $($arg:tt)+)?) => {{
        $crate::raw_print!($fmt $(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! println {
    ($fmt:expr $(, $($arg:tt)+)?) => {{
        $crate::raw_print!(concat!("\r", $fmt, "\r\n") $(, $($arg)+)?);
    }};
    () => {{
        $crate::print!("\r\n");
    }}
}
