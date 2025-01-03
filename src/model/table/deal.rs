use crate::{
    messages::{DealServerMessage, ServerMessage},
    model::{BlackjackTable, PlayerState},
};

impl BlackjackTable {
    /// Deals a new round, returning the appropriate message and a boolean if the deck was shuffled
    pub fn deal<'a>(&mut self) -> (ServerMessage<'a>, bool) {
        let mut hand_count = 0;
        for player in self.sitting_players() {
            if player.state() != PlayerState::PlayingThisRound {
                continue;
            }

            hand_count += player.hands().len();
        }

        let shoe = self.shoe.as_mut().unwrap();
        let mut shuffle = false;

        if shoe.should_shuffle() {
            shoe.shuffle(true);
            shuffle = true;
        }

        let (dealer_up, shuffled) = shoe.draw();
        shuffle |= shuffled;
        let (dealer_down, shuffled) = shoe.draw();
        shuffle |= shuffled;

        let mut hands = Vec::with_capacity(hand_count);
        for _ in 0..hand_count {
            let (card1, shuffled) = shoe.draw();
            shuffle |= shuffled;
            let (card2, shuffled) = shoe.draw();
            shuffle |= shuffled;

            hands.push((card1, card2));
        }

        (
            DealServerMessage::new((dealer_up, dealer_down), hands),
            shuffle,
        )
    }
}
