use super::PlayerView;

impl PlayerView {
    /// Create a new [`PlayerView`]
    pub fn new(width: usize) -> PlayerView {
        PlayerView {
            username: String::new(),
            ai: false,
            is_local: false,
            y: 0,
            width,
        }
    }
}
