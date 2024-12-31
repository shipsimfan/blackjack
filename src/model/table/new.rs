use crate::model::{BlackjackTable, GameState, Hand};
use std::num::{NonZeroU16, NonZeroU8};

impl BlackjackTable {
    /// Creates a new empty [`BlackjackTable`]
    pub fn new(
        max_players: usize,
        max_bet: NonZeroU16,
        min_bet: NonZeroU16,
        min_players: NonZeroU8,
        min_humans: u8,
        max_hands: NonZeroU8,
    ) -> Self {
        let mut players = Vec::with_capacity(max_players);
        for _ in 0..max_players {
            players.push(None);
        }

        BlackjackTable {
            players: players.into_boxed_slice(),
            state: GameState::WaitingForPlayers,
            dealer_hand: Hand::new(None),
            max_bet,
            min_bet,
            min_players,
            min_humans,
            max_hands,
        }
    }
}
