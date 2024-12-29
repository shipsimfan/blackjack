use super::View;
use crate::{MainGame, VirtualTerminal};
use blackjack::model::BlackjackTable;

impl MainGame {
    /// Creates a new [`MainGame`] based on `model`
    pub fn new(
        client_id: u8,
        table: BlackjackTable,
        server_name: String,
        terminal: &mut VirtualTerminal,
    ) -> Self {
        let view = View::new(&table, server_name, client_id as _, terminal);

        MainGame { table, view }
    }
}
