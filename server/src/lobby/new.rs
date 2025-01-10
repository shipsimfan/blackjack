use super::Lobby;
use crate::Options;
use blackjack::{model::BlackjackTable, pkg_version};
use std::collections::VecDeque;

impl Lobby {
    /// Creates a new [`Lobby`] from `options`
    pub fn new(options: Options) -> Self {
        let mut clients = Vec::with_capacity(options.max_players());
        for _ in 0..options.max_players() {
            clients.push(None);
        }

        Lobby {
            table: BlackjackTable::new(
                options.max_players(),
                options.max_bet,
                options.min_bet,
                options.min_players,
                options.min_humans,
                options.max_hands,
                options.decks,
                Vec::new(),
            ),
            clients: clients.into_boxed_slice(),
            connecting_clients: VecDeque::with_capacity(options.max_players()),
            connection_timeout: options.connection_timeout(),
            server_name: options
                .server_name
                .unwrap_or("a blackjack server".to_owned()),
            password: options.password,
            server_version: pkg_version!(),
            write_all_buffer: Vec::with_capacity(u16::MAX as usize),
        }
    }
}
