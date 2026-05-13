use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::shell::{
    core::{
        QueryResult, ShellAutoComplete, ShellCommandProvider, ShellInterpreter, ShellTokenizer,
    },
    Shell,
};

impl Shell {
    pub(crate) fn handle_tab<
        T,
        C,
        Interpreter: ShellInterpreter<T, C>,
        CommandProvider: ShellCommandProvider<T, C>,
        Tokenizer: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), std::io::Error> {
        let result = self
            .services
            .auto_complete
            .query_command::<T, C, CommandProvider>(&self.buffer)?;

        match result {
            QueryResult::NoMatch | QueryResult::Bell => {
                execute!(self.stdout, Print("\x07"))?;
            }
            QueryResult::CommonPrefix(cmd) => {
                self.buffer = cmd;

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(super::PREFIX),
                    Print(&self.buffer),
                    MoveToColumn((&self.buffer.len() + 2) as u16)
                )?;
            }
            QueryResult::ExactMatch(cmd) | QueryResult::SingleMatch(cmd) => {
                self.buffer = cmd + " ";

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(super::PREFIX),
                    Print(&self.buffer),
                    MoveToColumn((&self.buffer.len() + 2) as u16)
                )?;
            }
            QueryResult::MultipleMatches(matches) => {
                let results = matches.iter().fold(String::new(), |acc, s| acc + " " + s);

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(format!(
                        "{}{}\r\n{}\r\n{}{}",
                        super::PREFIX,
                        &self.buffer,
                        results.trim(),
                        super::PREFIX,
                        &self.buffer
                    )),
                    MoveToColumn((&self.buffer.len() + 2) as u16),
                )?;
            }
        }
        Ok(())
    }
}
