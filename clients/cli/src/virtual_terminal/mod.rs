use win32::HANDLE;

mod event;
mod special_key;
mod text_input;

mod cursor_display;
mod get;
mod move_cursor;
mod new;
mod read;
mod write;

pub use event::TerminalEvent;
pub use new::VirtualTerminalCreationError;
pub use special_key::SpecialKey;
pub use text_input::TextInput;

/// A representation of a virtual terminal
pub struct VirtualTerminal {
    /// The output handle for writing to the terminal
    output: HANDLE,

    /// The input handle for reading from the terminal
    input: HANDLE,

    /// The width of the terminal
    width: usize,

    /// The height of the terminal
    height: usize,
}
