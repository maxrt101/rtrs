use core::fmt::{Arguments, Write};
use core::marker::PhantomData;
use crate::log::{Level, Record, Severity};
use crate::log::console::ConsoleWriter;
use crate::util::traits::Empty;
use crate::sync::RwLock;
use crate::object_with;

pub struct Logger<R: Record, W: Write + Empty = ConsoleWriter> {
    name: &'static str,
    _writer: PhantomData<W>,
    _record: PhantomData<RwLock<R>>,
}

impl<R: Record, W: Write + Empty> Logger<R, W> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _writer: PhantomData, _record: PhantomData }
    }

    pub fn writer(&self) -> W {
        W::empty()
    }

    pub fn record(
        &self,
        severity: Severity,
        level:    Level,
        file:     &'static str,
        line:     u32,
        args:     Arguments
    ) -> impl Record {
        R::from(severity, level, self.name, file, line, args)
    }

    pub fn check(&self, severity: Severity, level: Level) -> bool {
        object_with!(crate::log::LOGGER_META_OBJECT_NAME, crate::log::meta::ModuleMetaManager, meta, {
            if let Some(meta) = meta.get(self.name) {
                if severity as u8 >= meta.severity as u8 && level <= meta.level {
                    true
                } else {
                    false
                }
            } else {
                meta.allow_unregistered()
            }
        })
    }
}

