use crate::{MainGame, VirtualTerminal};
use blackjack::model::BlackjackTable;

impl MainGame {
    /// Creates a new [`MainGame`] based on `model`
    pub fn new(model: BlackjackTable, terminal: &mut VirtualTerminal) -> Self {
        MainGame { model }
    }
}
