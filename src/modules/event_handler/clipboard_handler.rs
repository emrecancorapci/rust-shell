use crossterm::{execute, style::Print};

use crate::shell::Shell;

impl Shell {
    pub(crate) fn handle_clipboard(&mut self, clipboard: String) -> Result<(), std::io::Error> {
        execute!(self.stdout, Print(clipboard))?;
        Ok(())
    }
}
