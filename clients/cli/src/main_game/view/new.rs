use super::{
    controls::ControlsView, ChatView, PlayersView, View, MIN_HAND_LINE_LENGTH, VERTICAL_BAR_WIDTH,
};
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Creates a new [`View`] for `table`
    pub fn new(
        table: &BlackjackTable,
        server_name: String,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) -> Self {
        //terminal.use_alt_buffer();
        terminal.hide_cursor();

        let game_width = MIN_HAND_LINE_LENGTH + table.max_bet().ilog10() as usize + 1;
        let server_name_height = server_name.len().div_ceil(game_width);
        let dealer_hand_y = server_name_height + 4;

        let chat = ChatView::new(game_width + VERTICAL_BAR_WIDTH);
        let players =
            PlayersView::new(table.max_players(), dealer_hand_y + 2, game_width, local_id);
        let controls = ControlsView::new(game_width);

        let mut view = View {
            chat,
            players,
            controls,
            game_width,
            server_name,
            server_name_height,
            dealer_hand_y,
            min_bet: table.min_bet(),
            max_bet: table.max_bet(),
        };
        view.resize(table, local_id, terminal);
        view
    }
}
