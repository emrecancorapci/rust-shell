use crossterm::{cursor::MoveToColumn, execute, style::Print};

use crate::shell::{
    core::{ShellAutoComplete, ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};
use std::io::{Error, ErrorKind, Write};

impl Shell {
    pub(crate) fn handle_enter<
        T,
        Interpreter: ShellInterpreter<T>,
        CommandProvider: ShellCommandProvider<T>,
        Tokenizer: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        if !self.buffer.is_empty() {
            execute!(self.stdout, Print("\r\n"), MoveToColumn(0))?;

            let tokens = Tokenizer::tokenize(self.buffer.trim())?;

            match Interpreter::run::<CommandProvider>(&tokens) {
                Ok(bytes) => {
                    let output = str::from_utf8(&bytes).unwrap();
                    let formatted = output.replace('\n', "\r\n");

                    execute!(self.stdout, Print(&formatted), MoveToColumn(0))?;
                }
                Err(err) => {
                    if err.kind() == ErrorKind::Interrupted {
                        return Err(err);
                    }
                    self.stderr.write(err.to_string().as_bytes())?;
                }
            }
        }

        execute!(self.stdout, Print("\r\n"), Print(super::PREFIX))?;

        self.auto_complete.reset();
        self.buffer.clear();
        Ok(())
    }
}
