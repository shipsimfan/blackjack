use super::{chat::ChatView, View, MIN_HAND_LINE_LENGTH, VERTICAL_BAR_WIDTH};
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Creates a new [`View`] for `table`
    pub fn new(
        table: &BlackjackTable,
        server_name: String,
        terminal: &mut VirtualTerminal,
    ) -> Self {
        terminal.hide_cursor();

        let game_width = MIN_HAND_LINE_LENGTH + table.max_bet().ilog10() as usize + 1;
        let chat = ChatView::new(game_width + VERTICAL_BAR_WIDTH);

        let mut view = View {
            chat,
            game_width,
            server_name,
            min_bet: table.min_bet(),
            max_bet: table.max_bet(),
        };
        view.resize(table, terminal);
        view
    }
}
