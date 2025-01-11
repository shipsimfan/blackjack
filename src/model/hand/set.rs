use crate::model::Hand;

impl Hand {
    /// Set if this hand contains a hidden card
    pub(crate) fn set_hidden_card(&mut self, hidden_card: bool) {
        self.hidden_card = hidden_card;
    }
}
