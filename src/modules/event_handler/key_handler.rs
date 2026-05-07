use std::io::{Error, ErrorKind};

use crossterm::{
    cursor::{MoveLeft, MoveRight},
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
};

use crate::shell::{
    core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};

mod backspace;
mod ch;
mod enter;
mod tab;

const PREFIX: &str = "$ ";

impl Shell {
    pub fn handle_keys<
        T,
        I: ShellInterpreter<T>,
        C: ShellCommandProvider<T>,
        K: ShellTokenizer<T>,
    >(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<(), Error> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(key_event.code, KeyCode::Char('c')) {
                return Err(Error::new(ErrorKind::Interrupted, "ctrl-c"));
            } else if matches!(key_event.code, KeyCode::Char('j')) {
                return self.handle_enter::<T, I, C, K>();
            }
        }

        if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Char(ch) => self.handle_ch(ch)?,
                KeyCode::Enter => self.handle_enter::<T, I, C, K>()?,
                KeyCode::Tab => self.handle_tab::<T, I, C, K>()?,
                KeyCode::Backspace => self.handle_backspace()?,
                KeyCode::Left => {
                    let relative_cursor_x = self.cursor_x as usize - PREFIX.len();

                    if relative_cursor_x != 0 {
                        execute!(self.stdout, MoveLeft(1))?;
                        self.cursor_x = self.cursor_x - 1;
                    }
                }
                KeyCode::Right => {
                    let relative_cursor_x = self.cursor_x as usize - PREFIX.len();

                    if relative_cursor_x < self.buffer.len() {
                        execute!(self.stdout, MoveRight(1))?;
                        self.cursor_x = self.cursor_x + 1;
                    }
                }
                KeyCode::Up => return Ok(()), // TODO: Implement history
                KeyCode::Down => return Ok(()), // TODO: Implement history
                KeyCode::Delete => return Ok(()), // TODO: Implement delete functionality
                KeyCode::Home => todo!(),
                KeyCode::End => todo!(),
                KeyCode::PageUp => todo!(),
                KeyCode::PageDown => todo!(),
                KeyCode::BackTab => todo!(),
                KeyCode::Insert => todo!(),
                KeyCode::F(_) => todo!(),
                KeyCode::Null => todo!(),
                KeyCode::Esc => todo!(),
                KeyCode::CapsLock => todo!(),
                KeyCode::ScrollLock => todo!(),
                KeyCode::NumLock => todo!(),
                KeyCode::PrintScreen => todo!(),
                KeyCode::Pause => todo!(),
                KeyCode::Menu => todo!(),
                KeyCode::KeypadBegin => todo!(),
                KeyCode::Media(_) => todo!(),
                KeyCode::Modifier(_) => todo!(),
            }
        }

        Ok(())
    }
}
