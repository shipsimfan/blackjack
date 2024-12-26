use crate::{messages::Username, model::Player};

impl Player {
    /// Creates a new blackjack [`Player`]
    pub fn new(username: Username<'static>, ai: bool) -> Self {
        Player { username, ai }
    }
}
