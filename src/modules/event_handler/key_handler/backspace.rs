use crossterm::{
    cursor::{self, MoveLeft, MoveToColumn},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::{core::ShellAutoComplete, Shell};

impl Shell {
    pub(crate) fn handle_backspace(&mut self) -> Result<(), std::io::Error> {
        let relative_cursor_x = cursor::position()?.0 as usize - super::PREFIX.len();
        if relative_cursor_x == 0 {
            return Ok(());
        }

        self.buffer.remove(relative_cursor_x - 1);

        execute!(
            self.stdout,
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
            Print(super::PREFIX),
            Print(&self.buffer),
        )?;

        if relative_cursor_x <= self.buffer.len() {
            execute!(self.stdout, MoveLeft(1))?;
        }

        self.auto_complete.reset();
        Ok(())
    }
}
