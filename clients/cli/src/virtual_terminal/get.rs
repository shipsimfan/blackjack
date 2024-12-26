use crate::VirtualTerminal;
use win32::HANDLE;

impl VirtualTerminal {
    /// Gets the handle to the virtual terminal input
    pub fn input(&self) -> HANDLE {
        self.input
    }

    /// Get the number of columns in the terminal
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the terminal
    pub fn height(&self) -> usize {
        self.height
    }
}
