
#[macro_export]
macro_rules! bit_or {
    ($($flag:expr $(,)?)+) => {
        $( (1 << $flag as u32) | )+ 0
    };
}

#[macro_export]
macro_rules! bit_get {
    ($flags:expr, $flag:expr) => {
        $flags & (1 << $flag as u32) != 0
    }
}

#[macro_export]
macro_rules! bit_set {
    ($flags:expr, $flag:expr) => {
        $flags |= (1 << $flag as u32)
    }
}

#[macro_export]
macro_rules! bit_clear {
    ($flags:expr, $flag:expr) => {
        $flags &= !(1 << $flag as u32)
    }
}

#[macro_export]
macro_rules! bit_if {
    ($flags:expr, $flag:expr, $blk:expr) => {
        if $crate::bit_get!($flags, $flag) {
            $blk
        }
    }
}

#[macro_export]
macro_rules! bit_if_or_else {
    ($flags:expr, $flag:expr, $blk:expr, $blk_else:expr) => {
        if $crate::bit_get!($flags, $flag) {
            $blk
        } else {
            $blk_else
        }
    }
}