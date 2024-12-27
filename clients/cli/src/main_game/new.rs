use crate::{MainGame, VirtualTerminal};
use blackjack::model::BlackjackTable;

impl MainGame {
    /// Creates a new [`MainGame`] based on `model`
    pub fn new(client_id: u8, model: BlackjackTable, terminal: &mut VirtualTerminal) -> Self {
        MainGame { client_id, model }
    }
}
