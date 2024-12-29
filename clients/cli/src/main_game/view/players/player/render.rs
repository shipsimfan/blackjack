use super::PlayerView;
use crate::{
    main_game::view::{CONTROLS_HEIGHT, HAND_LINE_MARGIN},
    VirtualTerminal,
};
use blackjack::model::Player;

impl PlayerView {
    pub fn render(
        &mut self,
        y: usize,
        player: &Player,
        is_local: bool,
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

        // Render hands
        if y + 1 < terminal.height() - CONTROLS_HEIGHT {
            terminal.move_cursor_to(HAND_LINE_MARGIN, y + 1);
            terminal.write_blank(self.width - HAND_LINE_MARGIN);
        }

        // Set `render_blank_line` if hand size changed

        // Adjust height

        // Adjust passed variables
        self.ai = player.ai();
        self.y = y;
        self.is_local = is_local;

        // Render blank line if needed
        if render_blank_line && y + self.height() - 1 < terminal.height() - CONTROLS_HEIGHT {
            terminal.move_cursor_to(0, self.y + self.height() - 1);
            terminal.write_blank(self.width);
        }
    }
}
