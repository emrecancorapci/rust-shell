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
use std::{collections::HashSet, io::Error};

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

        let mut search_result = HashSet::new();

        let builtin_commands = CommandProvider::get_commands();
        let found_builtin_command = builtin_commands
            .iter()
            .find(|c| c.starts_with(&self.tab_query));

        if found_builtin_command.is_some() {
            search_result.insert(found_builtin_command.unwrap().to_string());
        }

        search_result.extend(CommandProvider::search_command(&self.tab_query)?);

        if search_result.len() == 0 {
            return Ok(());
        }

        if search_result.len() == 1 {
            self.buffer = search_result.iter().nth(0).unwrap().to_string() + " ";

            execute!(
                self.stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                Print(super::PREFIX),
                Print(&self.buffer),
                MoveToColumn((&self.buffer.len() + 2) as u16)
            )?;

            return Ok(());
        } else if self.tab_index >= 1 {
            let results = search_result
                .iter()
                .fold(String::new(), |acc, s| acc + " " + s);

            dbg!(&results);

            execute!(
                self.stdout,
                Clear(ClearType::CurrentLine),
                MoveToColumn(0),
                Print(super::PREFIX),
                Print(&self.buffer),
                Print("\r\n"),
                Print(results.trim()),
                Print("\r\n"),
                Print(super::PREFIX),
                Print(&self.buffer),
                MoveToColumn((&self.buffer.len() + 2) as u16),
            )?;
        }

        self.tab_index += 1;
        Ok(())
    }
}
