use super::PlayerView;

impl PlayerView {
    /// Create a new [`PlayerView`]
    pub fn new(width: usize) -> PlayerView {
        PlayerView {
            username: String::new(),
            hands: Vec::new(),
            ai: false,
            is_local: false,
            total_earnings: 0,
            last_round_earnings: None,
            not_playing: false,
            y: 0,
            width,
        }
    }
}
