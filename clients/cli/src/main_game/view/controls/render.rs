use super::{ControlState, ControlsView};
use crate::VirtualTerminal;
use blackjack::model::{BlackjackTable, GameState};
use std::borrow::Cow;

impl ControlsView {
    /// Render the controls if they have changed
    pub fn render(
        &mut self,
        force: bool,
        table: &BlackjackTable,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) {
        let state = ControlState::get(table, local_id);

        if !force
            && state == self.state
            && self.last_selected_option == self.selected_option
            && self.game_state == table.state()
        {
            return;
        }

        if state != self.state {
            self.selected_option = 0;
        }

        // Render game state
        if force || self.game_state != table.state() {
            self.game_state = table.state();

            let state_message = match self.game_state {
                GameState::WaitingForPlayers => Cow::Borrowed("Waiting for players..."),
                GameState::WaitingForBets => Cow::Borrowed("Waiting for bets..."),
                GameState::WaitingForPlayer(player, hand) => Cow::Owned(format!(
                    "Waiting for {} ({})...",
                    table.player(player as _).username(),
                    (hand as usize) + 1
                )),
            };

            let total_extra = self.width - state_message.len();
            let margin = total_extra / 2;
            terminal.move_cursor_to(0, self.y);
            terminal.write_blank(margin);
            terminal.write(state_message);
            terminal.write_blank(total_extra - margin);
        }

        // Render options
        terminal.move_cursor_to(0, self.y + 1);

        if state == ControlState::None {
            terminal.write_blank(self.width);
            self.state = state;
            return;
        }

        if state == ControlState::Betting {
            self.state = state;
            self.bet_input.render(!self.chat_active, terminal);
            return;
        }

        let options = state.options();
        let mut options_width = options.len() * 4 - 2;
        for option in options {
            options_width += option.len();
        }

        assert!(options_width <= self.width);
        let total_extra = self.width - options_width;
        let margin = total_extra / 2;

        terminal.write_blank(margin);
        for i in 0..options.len() {
            if i != 0 {
                terminal.write_blank(2);
            }

            if !self.chat_active && i == self.selected_option {
                terminal.write("\x1B[7m");
            }

            terminal.write_blank(1);
            terminal.write(options[i]);
            terminal.write_blank(1);

            if !self.chat_active && i == self.selected_option {
                terminal.write("\x1B[27m");
            }
        }
        terminal.write_blank(total_extra - margin);

        self.last_selected_option = self.selected_option;
        self.state = state;
    }
}
