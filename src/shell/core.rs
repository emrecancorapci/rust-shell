pub trait ShellInterpreter<T> {
    fn run<R: ShellCommandProvider<T>>(tokens: &[T]) -> Result<Vec<u8>, std::io::Error>;
}

pub trait ShellTokenizer<T> {
    fn tokenize(input: &str) -> Result<Vec<T>, std::io::Error>;
}

pub trait ShellCommandProvider<T> {
    fn run(cmd: &str, tokens: &[T]) -> Result<String, std::io::Error>;
    fn get_commands() -> Vec<&'static str>;
    fn search_command(query: &str) -> Result<Vec<String>, std::io::Error>;
}

pub trait ShellCommand<T> {
    fn run(tokens: &[T]) -> Result<String, std::io::Error>;
}

pub trait ShellAutoComplete {
    fn new() -> Self
    where
        Self: Sized;
    fn reset(&mut self);
    fn query_command<T, CommandProvider: ShellCommandProvider<T>>(
        &mut self,
        command: &str,
    ) -> Result<QueryResult, std::io::Error>;
}

pub trait ShellHistoryHandler {
    fn new() -> Self
    where
        Self: Sized;
    fn add_entry(&mut self, entry: &str);
    fn remove_entry(&mut self, index: usize) -> Result<(), Error>;
    fn get_previous(&mut self) -> Option<String>;
    fn get_next(&mut self) -> Option<String>;
    fn get_all(&self) -> Vec<String>;
    fn get_nth(&self, n: usize) -> Option<String>;
    fn clear(&mut self) -> Result<(), Error>;
    fn load(&mut self) -> Result<(), Error>;
    fn save(&self) -> Result<(), Error>;
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
