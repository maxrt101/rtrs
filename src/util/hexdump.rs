use core::fmt::{Display, Formatter};

use crate::{
    write_color_fmt,
    write_wrap_color_fmt,
    util::color::{
        WrapColorAction::*,
        WrapColorType::*
    }
};


pub struct Hexdump<'a> {
    buf: *const u8,
    size: usize,

    per_line: usize,
    offset_size: usize,

    alternating_color: &'a str,
    offset_color: &'a str,
    chars_color: &'a str,
}

impl<'a> Hexdump<'a> {
    const MAX_SIZE: usize = 32;

    pub const fn from_raw(buf: *const u8, size: usize) -> Self {
        Self {
            buf, size,

            per_line: 16,
            offset_size: 4,

            alternating_color: "",
            offset_color: "",
            chars_color: "",
        }
    }

    pub const fn from<T>(value: &T) -> Self {
        Self::from_raw(value as *const _ as *const u8, size_of_val(value))
    }

    pub fn per_line(mut self, per_line: usize) -> Self {
        self.per_line = per_line;
        self
    }

    #[deprecated(note = "Doesn't work right now")]
    pub fn offset_size(mut self, offset_size: usize) -> Self {
        self.offset_size = offset_size;
        self
    }

    pub fn alternating_color(mut self, alternating_color: &'a str) -> Self {
        self.alternating_color = alternating_color;
        self
    }

    pub fn offset_color(mut self, offset_color: &'a str) -> Self {
        self.offset_color = offset_color;
        self
    }

    pub fn chars_color(mut self, chars_color: &'a str) -> Self {
        self.chars_color = chars_color;
        self
    }

    pub fn default_color(mut self) -> Self {
        self.alternating_color = crate::ANSI_TEXT_BOLD;
        self.offset_color = crate::ANSI_COLOR_FG_MAGENTA;
        self.chars_color = crate::ANSI_COLOR_FG_CYAN;

        self
    }

    // TODO: Refactor and generalize into some impl of Display
    //       Maybe enum Color { Default, FgRed, BgRed, ... } (impl Display for Color)
    //       Maybe struct ColoredString<'a> { s: &'a str, color: Color } (impl Display for ColoredString)
    // fn write_format_color(&self, f: &mut Formatter<'_>, color: &str, wrap: WrapColorType, action: WrapColorAction, args: core::fmt::Arguments<'_>) -> core::fmt::Result {}

    fn format_pad(&self, f: &mut Formatter<'_>, i: usize) -> core::fmt::Result {
        let left = i % self.per_line;

        if left != 0 {
            for _ in 0..(self.per_line - left) {
                write!(f, "   ")?;
            }
        }

        Ok(())
    }

    fn format_end_line(&self, f: &mut Formatter<'_>, i: usize, chars: &mut [char; Hexdump::MAX_SIZE]) -> core::fmt::Result {
        if i == 0 {
            return Ok(());
        }

        self.format_pad(f, i)?;

        let chars_to_print = if i % self.per_line == 0 { self.per_line } else { i % self.per_line };

        write_color_fmt!(f, After, Apply, self.chars_color, "| ")?;

        for j in 0..chars_to_print {
            write!(f, "{}", chars[j])?;
            chars[j] = '.';
        }

        write_color_fmt!(f, Before, Reset, self.chars_color, "\r\n")
    }

    fn format(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        assert!(self.per_line <= Hexdump::MAX_SIZE, "Hexdump limits line size to {}", Hexdump::MAX_SIZE);

        let mut chars = ['.'; 32];

        let mut i = 0;

        while i < self.size {
            if i % self.per_line == 0 {
                self.format_end_line(f, i, &mut chars)?;

                // TODO: Configure offset format (4/6/8 symbols)
                write_wrap_color_fmt!(f, self.offset_color, "{:04x} | ", i)?;
            }

            let byte = unsafe { *((self.buf as usize + i) as *const u8) };

            if i % 2 == 0 {
                write!(f, "{:02x} ", byte)?;
            } else {
                write_wrap_color_fmt!(f, self.alternating_color, "{:02x} ", byte)?;
            }

            let c = byte as char;

            if c.is_ascii() && !c.is_control() {
                chars[i % self.per_line] = c;
            }

            i += 1;
        }

        self.format_end_line(f, i, &mut chars)
    }
}

impl<'a> Display for Hexdump<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.format(f)
    }
}
