use super::Lobby;
use crate::Options;
use blackjack::pkg_version;
use std::collections::VecDeque;

impl Lobby {
    /// Creates a new [`Lobby`] from `options`
    pub fn new(options: Options) -> Self {
        Lobby {
            connecting_clients: VecDeque::with_capacity(options.max_players()),
            connection_timeout: options.connection_timeout(),
            server_name: options
                .server_name
                .unwrap_or("A blackjack server".to_owned()),
            password: options.password,
            server_version: pkg_version!(),
        }
    }
}
