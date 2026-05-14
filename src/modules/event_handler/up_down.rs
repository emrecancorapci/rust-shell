use std::io::Error;

use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::{core::ShellHistoryHandler, Shell};

impl Shell {
    pub(crate) fn handle_up(&mut self) -> Result<(), Error> {
        match self.services.history_handler.get_previous() {
            Some(cmd) => {
                self.buffer = cmd;
                let new_cursor_pos_x = (&self.buffer.len() + super::PREFIX.len()) as u16;

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(super::PREFIX),
                    Print(&self.buffer),
                    MoveToColumn(new_cursor_pos_x)
                )?;

                self.cursor_x = new_cursor_pos_x;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub(crate) fn handle_down(&mut self) -> Result<(), Error> {
        match self.services.history_handler.get_next() {
            Some(cmd) => {
                self.buffer = cmd;
                let new_cursor_pos_x = (&self.buffer.len() + super::PREFIX.len()) as u16;

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(super::PREFIX),
                    Print(&self.buffer),
                    MoveToColumn(new_cursor_pos_x)
                )?;

                self.cursor_x = new_cursor_pos_x;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
