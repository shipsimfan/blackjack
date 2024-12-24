mod display;
mod from;

/// An error that can occur while creating the virtual terminal
#[derive(Debug)]
pub struct VirtualTerminalCreationError {
    inner: win32::Error,
}

impl std::error::Error for VirtualTerminalCreationError {}
