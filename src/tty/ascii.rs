
pub const ASCII_KEY_NUL: u8 = 0;
pub const ASCII_KEY_SOH: u8 = 1;
pub const ASCII_KEY_STX: u8 = 2;
pub const ASCII_KEY_ETX: u8 = 3;
pub const ASCII_KEY_EOT: u8 = 4;
pub const ASCII_KEY_ENQ: u8 = 5;
pub const ASCII_KEY_ACK: u8 = 6;
pub const ASCII_KEY_BEL: u8 = 7;
pub const ASCII_KEY_BS:  u8 = 8;
pub const ASCII_KEY_HT:  u8 = 9;
pub const ASCII_KEY_LF:  u8 = 10;
pub const ASCII_KEY_VT:  u8 = 11;
pub const ASCII_KEY_FF:  u8 = 12;
pub const ASCII_KEY_CR:  u8 = 13;
pub const ASCII_KEY_SO:  u8 = 14;
pub const ASCII_KEY_SI:  u8 = 15;
pub const ASCII_KEY_DC0: u8 = 16;
pub const ASCII_KEY_DC1: u8 = 17;
pub const ASCII_KEY_DC2: u8 = 18;
pub const ASCII_KEY_DC3: u8 = 19;
pub const ASCII_KEY_DC4: u8 = 20;
pub const ASCII_KEY_NAK: u8 = 21;
pub const ASCII_KEY_SYN: u8 = 22;
pub const ASCII_KEY_ETB: u8 = 23;
pub const ASCII_KEY_CAN: u8 = 24;
pub const ASCII_KEY_EM:  u8 = 25;
pub const ASCII_KEY_SUB: u8 = 26;
pub const ASCII_KEY_ESC: u8 = 27;
pub const ASCII_KEY_FS:  u8 = 28;
pub const ASCII_KEY_GS:  u8 = 29;
pub const ASCII_KEY_RS:  u8 = 30;
pub const ASCII_KEY_US:  u8 = 31;
pub const ASCII_KEY_DEL: u8 = 127;

pub const ASCII_KEY_SPACE: u8 = 32;
pub const ASCII_KEY_EXC_MARK: u8 = 33;
pub const ASCII_KEY_DOUBLE_QUOTE: u8 = 34;
pub const ASCII_KEY_HASH: u8 = 35;
pub const ASCII_KEY_DOLLAR: u8 = 36;
pub const ASCII_KEY_PERCENT: u8 = 37;
pub const ASCII_KEY_AMPERSAND: u8 = 38;
pub const ASCII_KEY_APOSTROPHE: u8 = 39;

pub fn is_enter(key: u8) -> bool {
    key == ASCII_KEY_CR || key == ASCII_KEY_LF
}

pub fn is_backspace(key: u8) -> bool {
    key == ASCII_KEY_BS || key == ASCII_KEY_DEL
}
