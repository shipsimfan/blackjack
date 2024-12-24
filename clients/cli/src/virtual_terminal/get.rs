use crate::VirtualTerminal;
use win32::HANDLE;

impl VirtualTerminal {
    /// Gets the handle to the virtual terminal input
    pub fn input(&self) -> HANDLE {
        self.input
    }
}
