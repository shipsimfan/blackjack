use crate::{
    messages::{DealServerMessage, ServerMessage, ShuffleServerMessage},
    model::{BlackjackTable, PlayerState},
};

impl BlackjackTable {
    /// Deals a new round, returning the appropriate messages for dealing and shuffling
    pub fn deal<'a>(
        &mut self,
        additional_client: Option<usize>,
    ) -> (ServerMessage<'a>, Option<ServerMessage<'a>>) {
        let mut hand_count = 0;
        for i in 0..self.players.len() {
            let player = match &self.players[i] {
                Some(player) => player,
                None => continue,
            };

            if player.state() == PlayerState::PlayingThisRound {
                hand_count += player.hands().len();
            }

            if let Some(additional_client) = additional_client {
                if additional_client == i {
                    hand_count += 1;
                }
            }
        }

        let shoe = self.shoe.as_mut().unwrap();
        self.dealer_hand.clear(&mut Some(shoe));
        for player in &mut self.players {
            if let Some(player) = player {
                player.clear_hands(Some(shoe));
            }
        }

        let mut shuffle = false;

        assert_eq!(
            shoe.current_cards(),
            shoe.size(),
            "Lost cards from the deck"
        );

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
            if shuffle {
                Some(ShuffleServerMessage::new())
            } else {
                None
            },
        )
    }
}
