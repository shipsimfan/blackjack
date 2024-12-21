use super::Lobby;
use std::time::{Duration, SystemTime};

impl Lobby {
    /// Get the time to wait for an event before timing out
    pub fn get_timeout(&self) -> Option<Duration> {
        let timeout = match self.connecting_clients.front() {
            Some(front) => front.timeout(),
            None => return None,
        };

        Some(
            timeout
                .duration_since(SystemTime::now())
                .unwrap_or(Duration::ZERO),
        )
    }
}
