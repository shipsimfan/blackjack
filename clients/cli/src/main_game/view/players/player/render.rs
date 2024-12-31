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
                terminal.write_blank(
                    self.width - self.username.len() - if player.ai() { 5 } else { 0 },
                );
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
