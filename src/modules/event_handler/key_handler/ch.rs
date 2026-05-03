use crossterm::{
    cursor::{self, MoveToColumn},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::Shell;
use std::io::Error;

impl Shell {
    pub(crate) fn handle_ch(&mut self, ch: char) -> Result<(), Error> {
        let relative_cursor_x = cursor::position()?.0 as usize - super::PREFIX.len();

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

        self.tab_index = 0;
        self.tab_query = String::new();
        Ok(())
    }
}
