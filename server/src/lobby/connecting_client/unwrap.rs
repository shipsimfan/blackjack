use super::ConnectingClient;
use crate::server::ClientWriter;

impl ConnectingClient {
    /// Unwraps this into its underlying [`ClientWriter`]
    pub fn unwrap(self) -> ClientWriter {
        self.writer
    }
}
