use super::ConnectingClient;

impl ConnectingClient {
    /// Disconnects this connecting client
    pub fn disconnect(&mut self) {
        self.writer.disconnect();
    }
}
