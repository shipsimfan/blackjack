use super::Lobby;
use crate::Options;
use std::collections::VecDeque;

impl Lobby {
    /// Creates a new [`Lobby`] from `options`
    pub fn new(options: Options) -> Self {
        Lobby {
            connecting_clients: VecDeque::with_capacity(options.max_players()),
            connection_timeout: options.connection_timeout(),
        }
    }
}
