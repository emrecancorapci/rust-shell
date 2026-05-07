use crossterm::event::{self, Event};

use crate::shell::{
    core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};

mod clipboard_handler;
mod key_handler;

impl Shell {
    pub(crate) fn handle_event<
        T,
        I: ShellInterpreter<T>,
        C: ShellCommandProvider<T>,
        K: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), std::io::Error> {
        match event::read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key_event) => self.handle_keys::<T, I, C, K>(key_event),
            Event::Mouse(_) => todo!(),
            Event::Paste(clipboard) => self.handle_clipboard(clipboard),
            Event::Resize(_, _) => Ok(()),
        }
    }
}
