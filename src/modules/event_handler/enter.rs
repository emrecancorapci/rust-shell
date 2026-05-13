use crossterm::{cursor::MoveToColumn, execute, style::Print};

use crate::{
    modules::service_container::ServiceContainer,
    shell::{
        core::{
            ShellAutoComplete, ShellCommandProvider, ShellHistoryHandler, ShellInterpreter,
            ShellTokenizer,
        },
        Shell,
    },
};
use std::{
    fmt::Debug,
    io::{Error, ErrorKind, Write},
};

impl Shell {
    pub(crate) fn handle_enter<
        T: Debug,
        Interpreter: ShellInterpreter<T, ServiceContainer>,
        CommandProvider: ShellCommandProvider<T, ServiceContainer>,
        Tokenizer: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        if !self.buffer.is_empty() {
            execute!(self.stdout, Print("\r\n"), MoveToColumn(0))?;

            self.services.history_handler.add_entry(&self.buffer);

            let tokens = Tokenizer::tokenize(self.buffer.trim());

            if tokens.is_err() {
                self.stderr
                    .write(tokens.err().unwrap().to_string().as_bytes())?;
                execute!(self.stdout, Print("\r\n"), Print(super::PREFIX))?;

                self.cursor_x = super::PREFIX.len() as u16;
                self.cursor_y = self.cursor_y + 1;
                self.services.auto_complete.reset();
                self.buffer.clear();

                return Ok(());
            }

            let tokens = tokens.unwrap();

            match Interpreter::run::<CommandProvider>(&tokens, &self.services) {
                Ok(bytes) if !bytes.is_empty() => {
                    let output = str::from_utf8(&bytes).unwrap();
                    output
                        .lines()
                        .map(|l| execute!(self.stdout, Print(l), MoveToColumn(0), Print("\r\n")))
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?;

                    execute!(self.stdout, Print(super::PREFIX))?;
                }
                Ok(_) => {
                    execute!(self.stdout, Print(super::PREFIX))?;
                }
                Err(err) => {
                    if err.kind() == ErrorKind::Interrupted {
                        return Err(err);
                    }

                    err.to_string()
                        .lines()
                        .map(|l| execute!(self.stderr, Print(l), MoveToColumn(0), Print("\r\n")))
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?;

                    execute!(self.stdout, Print(super::PREFIX))?;
                }
            }
        }

        self.cursor_x = super::PREFIX.len() as u16;
        self.cursor_y = self.cursor_y + 1;
        self.services.auto_complete.reset();
        self.buffer.clear();
        Ok(())
    }
}
