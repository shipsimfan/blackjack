use crate::VirtualTerminalCreationError;

impl From<win32::Error> for VirtualTerminalCreationError {
    fn from(inner: win32::Error) -> Self {
        VirtualTerminalCreationError { inner }
    }
}
