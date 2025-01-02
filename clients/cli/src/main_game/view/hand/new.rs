use super::HandView;

impl HandView {
    /// Creates a new [`HandView`]
    pub fn new(width: usize, max_bet: u16) -> Self {
        HandView {
            bet: None,
            cards: Vec::new(),
            y: 0,
            width,
            max_bet_length: 2 + max_bet.ilog10() as usize,
        }
    }
}
