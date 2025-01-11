use crate::{
    messages::{DealServerMessage, Generate},
    model::Rank,
};

impl Generate for DealServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.dealer_up_card.generate(output);
        self.dealer_down_card
            .filter(|card| {
                let mut count = self.dealer_up_card.rank.value() + card.rank.value();
                if self.dealer_up_card.rank == Rank::Ace {
                    count += 10;
                }
                if card.rank == Rank::Ace {
                    count += 10;
                }

                if count == 21 {
                    true
                } else {
                    false
                }
            })
            .generate(output);

        self.hands.generate(output);
    }
}
