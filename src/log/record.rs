use core::fmt::{Arguments, Display, Formatter};
use crate::log::{Severity, Level};


pub trait Record: Display {
    fn from(
        severity: Severity,
        level:    Level,
        module:   &'static str,
        file:     &'static str,
        line:     u32,
        args:     Arguments
    ) -> impl Record;
}

pub struct DefaultRecord<'a> {
    module:   &'static str,
    severity: Severity,
    args:     Arguments<'a>
}

impl<'a> Display for DefaultRecord<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use crate::tty::ansi::*;

        // TODO: Rewrite using util::color
        write!(
            f,
            "\r[{:?}][{}{}{}] {}\r\n",
            self.severity,
            ANSI_COLOR_FG_MAGENTA,
            self.module,
            ANSI_TEXT_RESET,
            self.args
        )
    }
}

impl Record for DefaultRecord<'_> {
    fn from(
        severity: Severity,
        _level:   Level,
        module:   &'static str,
        _file:    &'static str,
        _line:    u32,
        args:     Arguments
    ) -> impl Record {
        DefaultRecord { module, severity, args }
    }
}

unsafe impl Send for DefaultRecord<'_> {}
