use super::ConnectingClient;
use std::time::SystemTime;

impl ConnectingClient {
    /// Gets the id of the connecting client
    pub fn id(&self) -> usize {
        self.writer.id()
    }

    /// Gets the time when this client should timeout connecting
    pub fn timeout(&self) -> SystemTime {
        self.timeout
    }
}
