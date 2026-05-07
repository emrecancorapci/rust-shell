use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::{core::ShellAutoComplete, Shell};

impl Shell {
    pub(crate) fn handle_ch(&mut self, ch: char) -> Result<(), std::io::Error> {
        // TODO: Fix overflow when cursor is at the next line
        let relative_cursor_x = self.cursor_x as usize - super::PREFIX.len();

        if relative_cursor_x < self.buffer.len() {
            self.buffer.insert(relative_cursor_x, ch);

            execute!(
                self.stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                Print(super::PREFIX),
                Print(&self.buffer),
                MoveToColumn((relative_cursor_x + 3) as u16)
            )?;
        } else {
            self.buffer.push(ch);
            execute!(self.stdout, Print(ch))?;
        }

        self.cursor_x += 1;
        self.auto_complete.reset();
        Ok(())
    }
}
