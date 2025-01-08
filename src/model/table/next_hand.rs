use crate::model::{BlackjackTable, Hand, PlayerState};

/// Locate the next valid hand in `hands`, if there is one
fn next_valid_hand(hands: &[Hand]) -> Option<usize> {
    for (i, hand) in hands.iter().enumerate() {
        if hand.value().as_u8() < 21 {
            return Some(i);
        }
    }

    None
}

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
            if let Some(next_hand) = next_valid_hand(&player.hands()[current_hand + 1..]) {
                return Some((current_player, next_hand + current_hand + 1));
            }
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

            if player.state() == PlayerState::PlayingThisRound {
                if let Some(hand) = next_valid_hand(player.hands()) {
                    return Some((i, hand));
                }
            }
        }

        None
    }
}
