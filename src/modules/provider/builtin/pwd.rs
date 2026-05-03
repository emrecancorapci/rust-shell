use crate::{shell::core::ShellCommand, modules::tokenizer::Token};

pub struct Pwd {}

impl ShellCommand<Token> for Pwd {
    fn run(_: &[Token]) -> Result<String, std::io::Error> {
        match std::env::current_dir() {
            Ok(path) => Ok(format!("{}", path.to_str().unwrap())),
            Err(err) => Err(err),
        }
    }
}
