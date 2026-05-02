use crate::shell::{
    core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};
use std::io::{Error, Write};

impl Shell {
    pub(crate) fn handle_tab<
        T,
        Interpreter: ShellInterpreter<T>,
        CommandProvider: ShellCommandProvider<T>,
        Tokenizer: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        match CommandProvider::get_commands()
            .iter()
            .find(|c| c.starts_with(self.buffer.trim()))
        {
            Some(found_command) => {
                let rest_of_the_command = &found_command[self.buffer.trim().len()..];

                self.stdout.write(rest_of_the_command.as_bytes())?;
                self.buffer.push_str(rest_of_the_command);
            }
            None => {
                self.stdout.write(&[7])?;
            }
        }

        Ok(())
    }
}
