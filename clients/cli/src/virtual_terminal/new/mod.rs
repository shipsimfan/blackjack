use crate::VirtualTerminal;
use win32::{
    try_get_last_error, GetConsoleScreenBufferInfo, GetStdHandle, SetConsoleMode,
    CONSOLE_SCREEN_BUFFER_INFO, ENABLE_EXTENDED_FLAGS, ENABLE_PROCESSED_OUTPUT,
    ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_WINDOW_INPUT, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    WCHAR,
};

mod error;

pub use error::VirtualTerminalCreationError;

impl VirtualTerminal {
    /// Creates a new [`VirtualTerminal`]
    pub fn new() -> Result<Self, VirtualTerminalCreationError> {
        let input = try_get_last_error!(GetStdHandle(STD_INPUT_HANDLE))?;
        let output = try_get_last_error!(GetStdHandle(STD_OUTPUT_HANDLE))?;

        try_get_last_error!(SetConsoleMode(
            input,
            ENABLE_EXTENDED_FLAGS | ENABLE_WINDOW_INPUT
        ))?;
        try_get_last_error!(SetConsoleMode(
            output,
            ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING
        ))?;

        let mut terminal_info = CONSOLE_SCREEN_BUFFER_INFO::default();
        try_get_last_error!(GetConsoleScreenBufferInfo(output, &mut terminal_info))?;

        let width = terminal_info.size.x as usize;
        let height = terminal_info.size.y as usize;

        let mut terminal = VirtualTerminal {
            output,
            input,
            width,
            height,
            blank_line: vec![b' ' as WCHAR; width],
        };
        terminal.hide_cursor();
        Ok(terminal)
    }
}
