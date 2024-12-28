use super::PlayersView;

impl PlayersView {
    /// Remove all players from this view without clearing them
    pub fn truncate(&mut self) {
        self.players.truncate(0);
    }
}
