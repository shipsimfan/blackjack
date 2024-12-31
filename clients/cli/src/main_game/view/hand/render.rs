use super::HandView;
use crate::{main_game::view::HAND_LINE_MARGIN, virtual_terminal::VirtualTerminal};
use blackjack::model::Hand;

impl HandView {
    /// Renders `hand` to `terminal`
    pub fn render(&mut self, hand: &Hand, y: usize, terminal: &mut VirtualTerminal) {
        if self.y == y && self.bet == hand.bet() {
            return;
        }

        self.y = y;
        self.bet = hand.bet();

        terminal.move_cursor_to(0, y);
        terminal.write_blank(HAND_LINE_MARGIN);
        let mut written = HAND_LINE_MARGIN;

        terminal.write_blank(self.width - self.max_bet_length - written - 1);
        written = self.width - self.max_bet_length - 1;
        if let Some(bet) = self.bet {
            let bet_str = format!("${}", bet);
            written += bet_str.len();
            terminal.write(bet_str);
        }

        terminal.write_blank(self.width - written);
    }
}
