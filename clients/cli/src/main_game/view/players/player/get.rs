use super::PlayerView;

impl PlayerView {
    /// Gets the height of this player view
    pub fn height(&self) -> usize {
        self.hands.len() + 2
    }
}
