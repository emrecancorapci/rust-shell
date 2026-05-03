use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::{
    core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer},
    Shell,
};
use std::io::Error;

impl Shell {
    pub(crate) fn handle_tab<
        T,
        Interpreter: ShellInterpreter<T>,
        CommandProvider: ShellCommandProvider<T>,
        Tokenizer: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        if self.tab_query.is_empty() {
            self.tab_query = self.buffer.trim().to_string();
        }

        let mut search_result = vec![];

        let builtin_commands = CommandProvider::get_commands();
        let found_builtin_command = builtin_commands
            .iter()
            .find(|c| c.starts_with(&self.tab_query));

        if found_builtin_command.is_some() {
            search_result.push(found_builtin_command.unwrap().to_string());
        }

        search_result.extend(CommandProvider::search_command(&self.tab_query)?);

        if search_result.len() == 0 {
            return Ok(());
        }

        let key_option = search_result.get(self.tab_index as usize);

        if key_option.is_some() {
            let key = key_option.unwrap();

            self.buffer = key.to_string() + " ";

            execute!(
                self.stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                Print(super::PREFIX),
                Print(&self.buffer),
                MoveToColumn((&self.buffer.len() + 2) as u16)
            )?;
            if self.tab_index < search_result.len() as u8 {
                self.tab_index += 1;
            }
        }

        Ok(())
    }
}
