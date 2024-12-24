use crate::VirtualTerminalCreationError;

impl std::fmt::Display for VirtualTerminalCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to create the virtual terminal - {}", self.inner)
    }
}
