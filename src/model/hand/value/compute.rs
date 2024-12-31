use crate::model::{Card, HandValue, Rank};

impl HandValue {
    /// Compute the value of `cards`
    pub(crate) fn compute(cards: &[Card]) -> Self {
        let mut total = 0;
        let mut aces = 0;
        for card in cards {
            total += card.rank.value();

            if card.rank == Rank::Ace {
                aces += 1;
            }
        }

        if aces == 0 {
            HandValue::NoAce(total as _)
        } else if total <= 11 {
            HandValue::Soft(total as u8 + 10)
        } else {
            HandValue::Hard(total as _)
        }
    }
}
