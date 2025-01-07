use crate::model::{BlackjackTable, PlayerState};

impl BlackjackTable {
    /// Gets the next playable hand
    pub fn next_hand(&self) -> Option<(usize, usize)> {
        let (current_player, current_hand) = match self.current_hand() {
            Some(current_hand) => current_hand,
            None => return self.next_player(0),
        };

        let player = match &self.players[current_player] {
            Some(player) => player,
            None => return self.next_player(current_player + 1),
        };

        if player.hands().len() > current_hand + 1 {
            return Some((current_player, current_hand + 1));
        }

        self.next_player(current_player + 1)
    }

    /// Gets the next player's first hand
    fn next_player(&self, start: usize) -> Option<(usize, usize)> {
        if start >= self.players.len() {
            return None;
        }

        for i in start..self.players.len() {
            let player = match &self.players[i] {
                Some(player) => player,
                None => continue,
            };

            if player.state() == PlayerState::PlayingThisRound && player.hands().len() > 0 {
                return Some((i, 0));
            }
        }

        None
    }
}
