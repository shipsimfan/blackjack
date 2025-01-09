use super::PlayerView;
use crate::{
    main_game::view::{HandView, CONTROLS_HEIGHT},
    VirtualTerminal,
};
use blackjack::model::{Player, PlayerState};

impl PlayerView {
    pub fn render(
        &mut self,
        y: usize,
        player: &Player,
        is_local: bool,
        max_bet: u16,
        terminal: &mut VirtualTerminal,
    ) {
        let mut render_blank_line = self.y != y;

        // Clear if needed
        if self.y > y {
            for y in self.y..self.y + self.height() {
                if y >= terminal.height() - CONTROLS_HEIGHT {
                    continue;
                }

                terminal.move_cursor_to(0, y);
                terminal.write(self.width);
            }
        }

        // Render username
        let username_neq = self.username != &**player.username();
        if username_neq || player.ai() != self.ai || self.y != y || self.is_local != is_local {
            if username_neq {
                self.username = player.username().to_string();
            }

            if y < terminal.height() - CONTROLS_HEIGHT {
                terminal.move_cursor_to(0, y);
                if is_local {
                    terminal.write("\x1B[1m");
                }
                terminal.write(&self.username);
                if player.ai() {
                    terminal.write(" (AI)");
                }
                if is_local {
                    terminal.write("\x1B[22m");
                }
            }
        }

        // Render last round earnings
        let earnings_start = self.width
            - (player.total_earnings().abs().checked_ilog10().unwrap_or(0) as usize
                + 4
                + if player.total_earnings() < 0 { 1 } else { 0 });

        if y < terminal.height() - CONTROLS_HEIGHT
            && (self.last_round_earnings != player.last_round_earnings()
                || username_neq
                || self.ai != player.ai())
        {
            let mut written = self.username.len() + if player.ai() { 5 } else { 0 } + 1;
            terminal.move_cursor_to(written, y);
            if let Some(last_round_earnings) = player.last_round_earnings() {
                let last_round_earnings = if last_round_earnings >= 0 {
                    format!(" (${})", last_round_earnings)
                } else {
                    format!(" (-${})", -last_round_earnings)
                };
                written += last_round_earnings.len();
                terminal.write(last_round_earnings);
            }

            terminal.write_blank(earnings_start - written);
        }

        // Render total earnings
        if y < terminal.height() - CONTROLS_HEIGHT
            && (self.total_earnings != player.total_earnings() || self.total_earnings == 0)
        {
            terminal.move_cursor_to(earnings_start, y);
            if player.total_earnings() < 0 {
                terminal.write(format_args!("-${}", -player.total_earnings()));
            } else {
                terminal.write(format_args!("${}", player.total_earnings()));
            }
        }

        // Render player state
        let not_playing = player.state() == PlayerState::NotPlaying;
        if y < terminal.height() - CONTROLS_HEIGHT && self.not_playing != not_playing {
            terminal.move_cursor_to(self.width - 1, y);
            if not_playing {
                terminal.write('■');
            } else {
                terminal.write(' ');
            }
        }

        // Render hands
        render_blank_line |= self.hands.len() != player.hands().len();
        while self.hands.len() > player.hands().len() {
            self.hands.pop();
        }
        while self.hands.len() < player.hands().len() {
            self.hands.push(HandView::new(self.width, max_bet));
        }

        for i in 0..player.hands().len() {
            let y = y + i + 1;
            if y >= terminal.height() - CONTROLS_HEIGHT {
                break;
            }

            self.hands[i].render(&player.hands()[i], y, terminal);
        }

        // Adjust passed variables
        self.ai = player.ai();
        self.y = y;
        self.is_local = is_local;
        self.not_playing = not_playing;

        // Render blank line if needed
        if render_blank_line && y + self.height() - 1 < terminal.height() - CONTROLS_HEIGHT {
            terminal.move_cursor_to(0, self.y + self.height() - 1);
            terminal.write_blank(self.width);
        }
    }
}
