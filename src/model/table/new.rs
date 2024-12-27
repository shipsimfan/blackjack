use crate::model::BlackjackTable;
use std::num::NonZeroU16;

impl BlackjackTable {
    /// Creates a new empty [`BlackjackTable`]
    pub fn new(max_players: usize, max_bet: NonZeroU16, min_bet: NonZeroU16) -> Self {
        let mut players = Vec::with_capacity(max_players);
        for _ in 0..max_players {
            players.push(None);
        }

        BlackjackTable {
            players: players.into_boxed_slice(),
            max_bet,
            min_bet,
        }
    }
}
