use super::{player::PlayerView, PlayersView};
use crate::VirtualTerminal;
use blackjack::model::Player;

impl PlayersView {
    /// Renders any players that have changed since the previous render
    pub fn render(
        &mut self,
        players: &[Option<Player>],
        max_bet: u16,
        force: bool,
        terminal: &mut VirtualTerminal,
    ) {
        let mut y = self.y;
        let mut i = 0;
        for (id, player) in players.iter().enumerate() {
            if y >= terminal.height() - 2 {
                break;
            }

            let player = match player {
                Some(player) => player,
                None => continue,
            };

            if i == self.players.len() {
                self.players.push(PlayerView::new(self.width));
            }

            self.players[i].render(y, player, id == self.local_id, max_bet, force, terminal);

            y += self.players[i].height();
            i += 1;
        }

        // Clear missing players
        i = self.players.len() - i;
        while i > 0 {
            let mut player = self.players.pop().unwrap();
            player.clear(y, terminal);
            i -= 1;
        }
    }
}
