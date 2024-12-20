use super::Lobby;
use crate::Options;

impl Lobby {
    /// Creates a new [`Lobby`] from `options`
    pub fn new(options: Options) -> Self {
        Lobby {
            connecting_clients: Vec::with_capacity(options.max_players()),
            connection_timeout: options.connection_timeout(),
        }
    }
}
