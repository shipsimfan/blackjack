use crate::VirtualTerminal;

impl VirtualTerminal {
    /// Move the cursor `x` columns to the right and `y` lines down
    pub fn move_cursor(&mut self, x: isize, y: isize) {
        self.move_cursor_x(x);
        self.move_cursor_y(y);
    }

    /// Move the cursor `x` columns to the right
    pub fn move_cursor_x(&mut self, x: isize) {
        if x > 0 {
            self.write(format_args!("\x1B[{}C", x));
        } else if x < 0 {
            self.write(format_args!("\x1B[{}D", -x));
        }
    }

    /// Move the cursor `y` lines down
    pub fn move_cursor_y(&mut self, y: isize) {
        if y > 0 {
            self.write(format_args!("\x1B[{}B", y));
        } else if y < 0 {
            self.write(format_args!("\x1B[{}A", -y));
        }
    }

    /// Move the cursor to column `x`
    pub fn move_cursor_to(&mut self, x: usize, y: usize) {
        self.write(format_args!("\x1B[{};{}H", y, x));
    }

    /// Move the cursor to column `x`
    pub fn move_cursor_to_x(&mut self, x: usize) {
        self.write(format_args!("\x1B[{}G", x));
    }
}
