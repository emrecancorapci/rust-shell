use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::ShellCommand,
};

pub struct Pwd {}

impl ShellCommand<Token, ServiceContainer> for Pwd {
    fn run(_: &[Token], _: &ServiceContainer) -> Result<String, std::io::Error> {
        match std::env::current_dir() {
            Ok(path) => Ok(format!("{}", path.to_str().unwrap())),
            Err(err) => Err(err),
        }
    }
}
