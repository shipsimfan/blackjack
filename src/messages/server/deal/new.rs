use crate::{
    messages::{DealServerMessage, ServerMessage},
    model::Card,
};

impl DealServerMessage {
    /// Creates a new [`DealServerMessage`]
    pub fn new<'a>(dealer_hand: (Card, Card), hands: Vec<(Card, Card)>) -> ServerMessage<'a> {
        ServerMessage::Deal(DealServerMessage {
            dealer_up_card: dealer_hand.0,
            dealer_down_card: Some(dealer_hand.1),
            hands,
        })
    }
}
