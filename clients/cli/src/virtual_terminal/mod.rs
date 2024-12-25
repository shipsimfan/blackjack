use win32::HANDLE;

mod event;

mod get;
mod new;
mod read;
mod write;

pub use event::TerminalEvent;
pub use new::VirtualTerminalCreationError;

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
