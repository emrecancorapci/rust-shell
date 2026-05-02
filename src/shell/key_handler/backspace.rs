use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::Shell;
use std::io::Error;

impl Shell {
    pub(crate) fn handle_backspace(&mut self) -> Result<(), Error> {
        self.buffer.pop();

        execute!(
            self.stdout,
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
            Print(super::PREFIX),
            Print(&self.buffer)
        )?;

        Ok(())
    }
}
