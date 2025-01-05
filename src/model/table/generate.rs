use crate::{messages::Generate, model::BlackjackTable};

impl Generate for BlackjackTable {
    fn generate(&self, output: &mut Vec<u8>) {
        self.players.generate(output);
        self.state.generate(output);

        // TODO: Change to generate_dealer(); function
        self.dealer_hand.generate(output);
        self.max_bet.generate(output);
        self.min_bet.generate(output);
        self.min_players.generate(output);
        self.min_humans.generate(output);
        self.max_hands.generate(output);
    }
}
