use connecting_client::ConnectingClient;
use std::{collections::VecDeque, time::Duration};

mod connecting_client;

mod get_timeout;
mod new;
mod on_connect;
mod on_disconnect;
mod on_message;
mod on_server_full;
mod on_timeout;

/// A lobby of blackjack
pub struct Lobby {
    /// The clients currently connecting
    connecting_clients: VecDeque<ConnectingClient>,

    /// The length of time to wait before disconnecting a client
    connection_timeout: Duration,
}
