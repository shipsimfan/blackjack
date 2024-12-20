use connecting_client::ConnectingClient;
use std::time::Duration;

mod connecting_client;

mod new;
mod on_connect;

/// A lobby of blackjack
pub struct Lobby {
    /// The clients currently connecting
    connecting_clients: Vec<ConnectingClient>,

    /// The length of time to wait before disconnecting a client
    connection_timeout: Duration,
}
