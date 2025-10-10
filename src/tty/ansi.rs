
// pub const ANSI_ESC: u8 = 0x1b;

pub enum AnsiCursor {
    AnsiCursorHome,
    AnsiCursorMove(u16, u16),
    AnsiCursorMoveUp(u16),
    AnsiCursorMoveDown(u16),
    AnsiCursorMoveRight(u16),
    AnsiCursorMoveLeft(u16),
    // TODO: other
}

impl core::fmt::Display for AnsiCursor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            AnsiCursor::AnsiCursorHome                         => write!(f, "\x1b[H"),
            AnsiCursor::AnsiCursorMove(line, col) => write!(f, "\x1b[{};{}H", line, col),
            AnsiCursor::AnsiCursorMoveUp(lines)         => write!(f, "\x1b[{}A", lines),
            AnsiCursor::AnsiCursorMoveDown(lines)       => write!(f, "\x1b[{}B", lines),
            AnsiCursor::AnsiCursorMoveRight(lines)      => write!(f, "\x1b[{}C", lines),
            AnsiCursor::AnsiCursorMoveLeft(lines)       => write!(f, "\x1b[{}D", lines),
        }
    }
}

pub const ANSI_TEXT_RESET:               &str = "\x1b[0m";
pub const ANSI_TEXT_BOLD:                &str = "\x1b[1m";
pub const ANSI_TEXT_RESET_BOLD:          &str = "\x1b[22m";
pub const ANSI_TEXT_DIM:                 &str = "\x1b[2m";
pub const ANSI_TEXT_RESET_DIM:           &str = "\x1b[22m";
pub const ANSI_TEXT_ITALIC:              &str = "\x1b[3m";
pub const ANSI_TEXT_RESET_ITALIC:        &str = "\x1b[23m";
pub const ANSI_TEXT_UNDERLINE:           &str = "\x1b[4m";
pub const ANSI_TEXT_RESET_UNDERLINE:     &str = "\x1b[24m";
pub const ANSI_TEXT_BLINK:               &str = "\x1b[5m";
pub const ANSI_TEXT_RESET_BLINK:         &str = "\x1b[25m";
pub const ANSI_TEXT_INVERSE:             &str = "\x1b[7m";
pub const ANSI_TEXT_RESET_INVERSE:       &str = "\x1b[27m";
pub const ANSI_TEXT_HIDDEN:              &str = "\x1b[8m";
pub const ANSI_TEXT_RESET_HIDDEN:        &str = "\x1b[28m";
pub const ANSI_TEXT_STRIKETHROUGH:       &str = "\x1b[9m";
pub const ANSI_TEXT_RESET_STRIKETHROUGH: &str = "\x1b[29m";

pub const ANSI_ERASE_IN_DISPLAY:                    &str = "\x1b[J";
pub const ANSI_ERASE_FROM_CURSOR_TO_SCREEN_END:     &str = "\x1b[0J";
pub const ANSI_ERASE_FROM_CURSOR_TO_SCREEN_START:   &str = "\x1b[1J";
pub const ANSI_ERASE_SCREEN:                        &str = "\x1b[2J";
pub const ANSI_ERASE_SAVED_LINES:                   &str = "\x1b[3J";
pub const ANSI_ERASE_IN_LINE:                       &str = "\x1b[K";
pub const ANSI_ERASE_FROM_CURSOR_TO_LINE_END:       &str = "\x1b[0K";
pub const ANSI_ERASE_FROM_CURSOR_TO_LINE_START:     &str = "\x1b[1K";
pub const ANSI_ERASE_LINE:                          &str = "\x1b[2K";

pub const ANSI_COLOR_FG_BLACK:   &str = "\x1b[30m";
pub const ANSI_COLOR_FG_RED:     &str = "\x1b[31m";
pub const ANSI_COLOR_FG_GREEN:   &str = "\x1b[32m";
pub const ANSI_COLOR_FG_YELLOW:  &str = "\x1b[33m";
pub const ANSI_COLOR_FG_BLUE:    &str = "\x1b[34m";
pub const ANSI_COLOR_FG_MAGENTA: &str = "\x1b[35m";
pub const ANSI_COLOR_FG_CYAN:    &str = "\x1b[36m";
pub const ANSI_COLOR_FG_WHITE:   &str = "\x1b[37m";
pub const ANSI_COLOR_FG_DEFAULT: &str = "\x1b[39m";

pub const ANSI_COLOR_BG_BLACK:   &str = "\x1b[40m";
pub const ANSI_COLOR_BG_RED:     &str = "\x1b[41m";
pub const ANSI_COLOR_BG_GREEN:   &str = "\x1b[42m";
pub const ANSI_COLOR_BG_YELLOW:  &str = "\x1b[43m";
pub const ANSI_COLOR_BG_BLUE:    &str = "\x1b[44m";
pub const ANSI_COLOR_BG_MAGENTA: &str = "\x1b[45m";
pub const ANSI_COLOR_BG_CYAN:    &str = "\x1b[46m";
pub const ANSI_COLOR_BG_WHITE:   &str = "\x1b[47m";
pub const ANSI_COLOR_BG_DEFAULT: &str = "\x1b[49m";
