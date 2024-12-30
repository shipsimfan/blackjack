use super::{ControlState, ControlsView};
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

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

        if !force && state == self.state && self.last_selected_option == self.selected_option {
            return;
        }

        if state != self.state {
            self.selected_option = 0;
        }

        terminal.move_cursor_to(0, self.y);

        if state == ControlState::None {
            terminal.write_blank(self.width);
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

            if i == self.selected_option {
                terminal.write("\x1B[7m");
            }

            terminal.write_blank(1);
            terminal.write(options[i]);
            terminal.write_blank(1);

            if i == self.selected_option {
                terminal.write("\x1B[27m");
            }
        }
        terminal.write_blank(total_extra - margin);

        self.last_selected_option = self.selected_option;
        self.state = state;
    }
}
