use core::fmt::Formatter;

pub enum WrapColorAction {
    Apply,
    Reset,
}

pub enum WrapColorType {
    Wrap,
    Before,
    After,
}

pub fn write_color(f: &mut Formatter<'_>, color: &str, wrap: WrapColorType, action: WrapColorAction, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
    if matches!(wrap, WrapColorType::Wrap) && matches!(action, WrapColorAction::Reset) {
        // Can't have Wrap & Reset
        return Err(core::fmt::Error);
    }

    if color.is_empty() {
        f.write_fmt(args)?;
    } else {
        let color = match action {
            WrapColorAction::Apply => color,
            WrapColorAction::Reset => crate::ANSI_TEXT_RESET,
        };

        match wrap {
            WrapColorType::Wrap => {
                f.write_str(color)?;
                f.write_fmt(args)?;
                f.write_str(crate::ANSI_TEXT_RESET)?;
            },
            WrapColorType::Before => {
                f.write_str(color)?;
                f.write_fmt(args)?;
            },
            WrapColorType::After => {
                f.write_fmt(args)?;
                f.write_str(color)?;
            },
        }
    }

    Ok(())
}

// TODO: Divide logic inside write_color into smaller functions
//       that will get called based on macro input

#[macro_export]
macro_rules! write_wrap_color_fmt {
    ($f:expr, $color:expr, $fmt:expr $(, $($arg:tt)+ )?) => {
        $crate::util::color::write_color($f, $color, $crate::util::color::WrapColorType::Wrap, $crate::util::color::WrapColorAction::Apply, format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! write_color_fmt {
    ($f:expr, $wrap:expr, $action:expr, $color:expr, $fmt:expr $(, $($arg:tt)+ )?) => {
        $crate::util::color::write_color($f, $color, $wrap, $action, format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! colored {
    ($color:expr, $str:expr) => {
        format_args!("{}{}{}", $color, $str, $crate::tty::ansi::ANSI_TEXT_RESET)
    };
}

#[macro_export]
macro_rules! colored_fmt {
    ($color:expr, $fmt:expr $(, $($arg:tt)+ )?) => {
        format_args!(concat!("{}", $fmt, "{}"), $color $(, $($arg)+)?, $crate::tty::ansi::ANSI_TEXT_RESET)
    };
}

#[deprecated(note = "Unusable")]
#[macro_export]
macro_rules! multicolored {
    ( ( $($color:expr),+ ), $str:expr ) => {
        // format_args!(concat!($str, "{}") $(, $color)+, crate::tty::ansi::ANSI_TEXT_RESET)
        format_args!("{}{}{}", $($color, )+ $str, $crate::tty::ansi::ANSI_TEXT_RESET)
    };
}
