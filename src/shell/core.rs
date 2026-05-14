use std::io::Error;

pub trait ShellInterpreter<T, C> {
    fn run<R: ShellCommandProvider<T, C>>(tokens: &[T], services: &mut C) -> Result<Vec<u8>, Error>;
}

pub trait ShellTokenizer<T> {
    fn tokenize(input: &str) -> Result<Vec<T>, Error>;
}

pub trait ShellTokenSerializer {
    fn serialize(self: &Self, skip: usize) -> String;
}

pub trait ShellCommandProvider<T, C> {
    fn run(cmd: &str, tokens: &[T], services: &mut C) -> Result<String, Error>;
    fn get_commands() -> Vec<&'static str>;
    fn search_command(query: &str) -> Result<Vec<String>, Error>;
}

pub trait ShellCommand<T, C> {
    fn run(tokens: &[T], services: &mut C) -> Result<String, Error>;
}

pub trait ShellAutoComplete {
    fn new() -> Self
    where
        Self: Sized;
    fn reset(&mut self);
    fn query_command<T, C, CommandProvider: ShellCommandProvider<T, C>>(
        &mut self,
        command: &str,
    ) -> Result<QueryResult, Error>;
}

pub trait ShellHistoryHandler {
    fn new() -> Self
    where
        Self: Sized;
    fn add_entry(&mut self, entry: &str);
    fn remove_entry(&mut self, index: usize) -> Result<(), Error>;
    fn get_previous(&mut self) -> Option<String>;
    fn get_next(&mut self) -> Option<String>;
    fn get_all(&self) -> &Vec<String>;
    fn get_nth(&self, n: usize) -> Option<String>;
    fn clear(&mut self) -> Result<(), Error>;
    fn load(&mut self) -> Result<(), Error>;
    fn load_from(&mut self, file_path: std::path::PathBuf) -> Result<(), Error>;
    fn save(&self) -> Result<(), Error>;
    fn save_to(&self, file_path: std::path::PathBuf) -> Result<(), Error>;
    fn append_to(&mut self, file_path: std::path::PathBuf) -> Result<(), Error>;
    fn reset_index(&mut self);
    fn set_default_path(&mut self, path: String) -> Result<(), Error>;
}

#[derive(PartialEq, Eq, Debug)]
pub enum QueryResult {
    NoMatch,
    Bell,
    ExactMatch(String),
    SingleMatch(String),
    CommonPrefix(String),
    MultipleMatches(Vec<String>),
}
