use crate::{
    messages::Username,
    model::{Player, PlayerState},
};

impl Player {
    /// Creates a new blackjack [`Player`]
    pub fn new(username: Username<'static>, ai: bool) -> Self {
        Player {
            username,
            ai,
            state: PlayerState::NotPlaying,
            hands: Vec::new(),
            total_earnings: 0,
            last_round_earnings: None,
        }
    }
}
