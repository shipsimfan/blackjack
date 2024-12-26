use crate::server::ClientWriter;
use blackjack::{messages::Version, model::BlackjackTable};
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
    /// The current state of the game
    table: BlackjackTable,

    /// The clients connected to the game
    clients: Box<[Option<ClientWriter>]>,

    /// The clients currently connecting
    connecting_clients: VecDeque<ConnectingClient>,

    /// The length of time to wait before disconnecting a client
    connection_timeout: Duration,

    /// The name of this server to report to clients
    server_name: String,

    /// The password to require clients to enter to connect
    password: Option<String>,

    /// The version of this server
    server_version: Version,
}
