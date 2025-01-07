use crate::model::{BlackjackTable, GameState, Hand, Player};

impl BlackjackTable {
    /// Gets all player slots
    pub fn players(&self) -> &[Option<Player>] {
        &self.players
    }

    /// Gets only the players seated at the table
    pub fn sitting_players(&self) -> impl Iterator<Item = &Player> {
        self.players.iter().filter_map(Option::as_ref)
    }

    /// Gets the player with `id`
    pub fn player(&self, id: usize) -> &Player {
        self.players[id].as_ref().unwrap()
    }

    /// Gets the maximum number of allowed players at this table
    pub fn max_players(&self) -> usize {
        self.players.len()
    }

    /// Gets the maximum bet a player can place at this table
    pub fn max_bet(&self) -> u16 {
        self.max_bet.get()
    }

    /// Gets the minimum bet a player can place at this table
    pub fn min_bet(&self) -> u16 {
        self.min_bet.get()
    }

    /// Gets the minimum number of players to start a round
    pub fn min_players(&self) -> u8 {
        self.min_players.get()
    }

    /// Gets the minimum number of human players to start a round
    pub fn min_humans(&self) -> u8 {
        self.min_humans
    }

    /// Gets the current state of the table
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Get the current player the game is waiting on
    pub fn current_player(&self) -> Option<usize> {
        match self.state {
            GameState::WaitingForPlayer(player, _) => Some(player as _),
            _ => return None,
        }
    }

    /// Get the current player and hand the game is waiting on
    pub fn current_hand(&self) -> Option<(usize, usize)> {
        match self.state {
            GameState::WaitingForPlayer(player, hand) => Some((player as _, hand as _)),
            _ => return None,
        }
    }

    /// Gets the dealer's hand
    pub fn dealer(&self) -> &Hand {
        &self.dealer_hand
    }

    /// Gets the player with `id` as a mutable reference
    pub(crate) fn player_mut(&mut self, id: usize) -> &mut Player {
        self.players[id].as_mut().unwrap()
    }
}
