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
