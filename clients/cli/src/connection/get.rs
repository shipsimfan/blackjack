use crate::Connection;
use win32::HANDLE;

impl Connection {
    /// Gets the event to wait on for read events
    pub fn read_event(&self) -> HANDLE {
        self.read_event
    }
}
