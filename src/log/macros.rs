
#[macro_export]
macro_rules! logger {
    ($name:expr) => {
        static LOGGER: $crate::log::Logger<$crate::log::DefaultRecord> = $crate::log::Logger::new($name);
    };

    ($name:expr, $record:ty) => {
        static LOGGER: $crate::log::Logger<$record> = $crate::log::Logger::new($name);
    };
}


#[macro_export]
macro_rules! log {
    ($severity:expr, $level:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        let args = format_args!($fmt $(, $($arg)+)?);
        $crate::log::log(&self::LOGGER, $severity, $level, file!(), line!(), args);
    }};
    
    (logger : $logger:expr, $severity:expr, $level:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        let args = format_args!($fmt $(, $($arg)+)?);
        $crate::log::log($logger, $severity, $level, file!(), line!(), args);
    }};
}

// TODO: Create macro that generates all of this macros

#[macro_export]
macro_rules! trace {
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Trace, 0, $fmt $(, $($arg)+)?);
    }};

    (level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Trace, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger : $logger, $crate::log::Severity::Trace, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger: $logger, $crate::log::Severity::Trace, $level, $fmt $(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Info, 0, $fmt $(, $($arg)+)?);
    }};

    (level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Info, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger : $logger, $crate::log::Severity::Info, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger: $logger, $crate::log::Severity::Info, $level, $fmt $(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Warn, 0, $fmt $(, $($arg)+)?);
    }};

    (level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Warn, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger : $logger, $crate::log::Severity::Warn, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger: $logger, $crate::log::Severity::Warn, $level, $fmt $(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Error, 0, $fmt $(, $($arg)+)?);
    }};

    (level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Error, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger : $logger, $crate::log::Severity::Error, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger: $logger, $crate::log::Severity::Error, $level, $fmt $(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! fatal {
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Fatal, 0, $fmt $(, $($arg)+)?);
    }};

    (level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!($crate::log::Severity::Fatal, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger : $logger, $crate::log::Severity::Fatal, $level, $fmt $(, $($arg)+)?);
    }};

    (logger : $logger:expr, level : $level:literal, $fmt:literal $(, $($arg:tt)+)?) => {{
        $crate::log!(logger: $logger, $crate::log::Severity::Fatal, $level, $fmt $(, $($arg)+)?);
    }};
}
