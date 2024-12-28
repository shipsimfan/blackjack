use super::PlayersView;

impl PlayersView {
    /// Creates a new view for players
    pub fn new(max_players: usize, y: usize, width: usize, local_id: usize) -> Self {
        PlayersView {
            players: Vec::with_capacity(max_players),
            y,
            width,
            local_id,
        }
    }
}
