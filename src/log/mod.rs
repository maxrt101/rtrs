use core::fmt::{Arguments, Debug, Display, Formatter, Write};
use crate::object_with_mut;

pub mod console;
pub mod meta;
pub mod record;
pub mod macros;
pub mod logger;
mod types;

pub use logger::Logger;
pub use record::{Record, DefaultRecord};
use crate::util::traits::Empty;

pub const LOGGER_META_OBJECT_NAME: &str = "logs";

#[derive(Copy, Clone)]
pub enum Severity {
    Trace,
    Info,
    Warn,
    Error,
    Fatal,
}

pub type Level = u8;

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Severity::Trace => write!(f, "trace"),
            Severity::Info  => write!(f, "info "),
            Severity::Warn  => write!(f, "warn "),
            Severity::Error => write!(f, "error"),
            Severity::Fatal => write!(f, "fatal"),
        }
    }
}

impl Debug for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use crate::tty::ansi::*;

        match self {
            Severity::Trace => write!(f, "{}trace{}", ANSI_COLOR_FG_CYAN, ANSI_TEXT_RESET),
            Severity::Info  => write!(f, "{}info {}", ANSI_COLOR_FG_BLUE, ANSI_TEXT_RESET),
            Severity::Warn  => write!(f, "{}warn {}", ANSI_COLOR_FG_YELLOW, ANSI_TEXT_RESET),
            Severity::Error => write!(f, "{}error{}", ANSI_COLOR_FG_RED, ANSI_TEXT_RESET),
            Severity::Fatal => write!(f, "{}fatal{}", ANSI_COLOR_BG_RED, ANSI_TEXT_RESET),
        }
    }
}

impl From<&str> for Severity {
    fn from(value: &str) -> Self {
        match value {
            "trace" => Severity::Trace,
            "info"  => Severity::Info,
            "warn"  => Severity::Warn,
            "error" => Severity::Error,
            "fatal" => Severity::Fatal,
            _       => Severity::Trace,
        }
    }
}

pub fn register(name: &'static str, severity: Severity, level: Level) {
    object_with_mut!(LOGGER_META_OBJECT_NAME, meta::ModuleMetaManager, meta, {
        meta.register(name, severity, level);
    })
}

pub fn log<R: Record, W: Write + Empty>(
    logger: &Logger<R, W>,
    severity: Severity,
    level: Level,
    file: &'static str,
    line: u32,
    args: Arguments
) {
    if logger.check(severity, level) {
        let mut writer = logger.writer();
        let record = logger.record(severity, level, file, line, args);
        write!(&mut writer, "{}", record).unwrap();
    }
}