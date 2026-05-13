use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
};

use crate::shell::{
    core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};
const PREFIX: &str = "$ ";

mod backspace;
mod ch;
mod enter;
mod tab;

impl Shell {
    pub(crate) fn handle_event<
        Token: Debug,
        Interpreter: ShellInterpreter<Token>,
        CommandProvider: ShellCommandProvider<Token>,
        Tokenizer: ShellTokenizer<Token>,
    >(
        &mut self,
    ) -> Result<(), std::io::Error> {
        match event::read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key_event) => {
                self.handle_keys::<Token, Interpreter, CommandProvider, Tokenizer>(key_event)
            }
            Event::Mouse(_) => todo!(),
            Event::Paste(clipboard) => self.handle_clipboard(clipboard),
            Event::Resize(_, _) => Ok(()),
        }
    }

    pub fn handle_keys<
        Token: Debug,
        Interpreter: ShellInterpreter<Token>,
        CommandProvider: ShellCommandProvider<Token>,
        Tokenizer: ShellTokenizer<Token>,
    >(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<(), Error> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(key_event.code, KeyCode::Char('c')) {
                return Err(Error::new(ErrorKind::Interrupted, "ctrl-c"));
            } else if matches!(key_event.code, KeyCode::Char('j')) {
                return self.handle_enter::<Token, Interpreter, CommandProvider, Tokenizer>();
            }
        }

        if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Char(ch) => self.handle_ch(ch)?,
                KeyCode::Enter => {
                    self.handle_enter::<Token, Interpreter, CommandProvider, Tokenizer>()?
                }
                KeyCode::Tab => self
                    .handle_tab::<Token, Interpreter, CommandProvider, Tokenizer>(
                    )?,
                KeyCode::Backspace => self.handle_backspace()?,
                KeyCode::Left => {
                    let relative_cursor_x = self.cursor_x as usize - PREFIX.len();

                    if relative_cursor_x != 0 {
                        execute!(self.stdout, cursor::MoveLeft(1))?;
                        self.cursor_x = self.cursor_x - 1;
                    }
                }
                KeyCode::Right => {
                    let relative_cursor_x = self.cursor_x as usize - PREFIX.len();

                    if relative_cursor_x < self.buffer.len() {
                        execute!(self.stdout, cursor::MoveRight(1))?;
                        self.cursor_x = self.cursor_x + 1;
                    }
                }
                KeyCode::Up => return Ok(()), // TODO: Implement history
                KeyCode::Down => return Ok(()), // TODO: Implement history
                KeyCode::Delete => return Ok(()), // TODO: Implement delete functionality
                KeyCode::Esc => return Ok(()),
                KeyCode::Home => todo!(),
                KeyCode::End => todo!(),
                KeyCode::PageUp => todo!(),
                KeyCode::PageDown => todo!(),
                KeyCode::BackTab => todo!(),
                KeyCode::Insert => todo!(),
                KeyCode::F(_) => todo!(),
                KeyCode::Null => todo!(),
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

    fn handle_clipboard(&mut self, clipboard: String) -> Result<(), std::io::Error> {
        execute!(self.stdout, Print(clipboard))?;
        Ok(())
    }
}
