use super::HandView;
use crate::{main_game::view::HAND_LINE_MARGIN, virtual_terminal::VirtualTerminal};
use blackjack::model::Hand;

impl HandView {
    /// Renders `hand` to `terminal`
    pub fn render(&mut self, hand: &Hand, y: usize, force: bool, terminal: &mut VirtualTerminal) {
        let cards_eq = self.cards == hand.cards();
        if self.y == y
            && self.bet == hand.bet()
            && cards_eq
            && self.hidden_card == hand.hidden_card()
            && !force
        {
            return;
        }

        self.y = y;
        self.bet = hand.bet();
        self.hidden_card = hand.hidden_card();
        if !cards_eq {
            self.cards = hand.cards().to_vec();
        }

        // Render margin
        terminal.move_cursor_to(0, y);
        terminal.write_blank(HAND_LINE_MARGIN);
        let mut written = HAND_LINE_MARGIN;

        // Render hand value
        if hand.cards().len() > 0 {
            let hand_value = hand.value().to_string();
            written += hand_value.len();
            terminal.write(hand_value);
        }

        terminal.write_blank(HAND_LINE_MARGIN + 4 - written);
        written = HAND_LINE_MARGIN + 4;

        // Render cards
        if self.hidden_card {
            terminal.write('🂠');
            written += 1;
        }

        for card in hand.cards() {
            terminal.write(card);
            written += 1;
        }

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
