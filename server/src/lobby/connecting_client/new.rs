use super::ConnectingClient;
use crate::server::ClientWriter;
use std::time::{Duration, SystemTime};

impl ConnectingClient {
    /// Creates a new [`ConnectingClient`]
    pub fn new(writer: ClientWriter, timeout: Duration) -> Self {
        let timeout = SystemTime::now() + timeout;

        ConnectingClient { writer, timeout }
    }
}
