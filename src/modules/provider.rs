use std::{
    env,
    io::{Error, ErrorKind},
};

use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::{ShellCommand, ShellCommandProvider},
};

use builtin::{cd::Cd, echo::Echo, exit::Exit, history::History, pwd::Pwd, type_::Type};

pub mod builtin;

pub const SUPPORTED_COMMANDS: [&str; 6] = ["echo", "type", "exit", "pwd", "cd", "history"];

pub struct CommandProvider {}

impl ShellCommandProvider<Token, ServiceContainer> for CommandProvider {
    fn run(cmd: &str, tokens: &[Token], services: &mut ServiceContainer) -> Result<String, Error> {
        match cmd {
            "echo" => Echo::run(tokens, services),
            "history" => History::run(tokens, services),
            "type" => Type::run(tokens, services),
            "exit" => Exit::run(tokens, services),
            "pwd" => Pwd::run(tokens, services),
            "cd" => Cd::run(tokens, services),
            _ => Err(Error::new(
                ErrorKind::NotFound,
                format!("{}: command not found", cmd),
            )),
        }
    }

    fn get_commands() -> Vec<&'static str> {
        return SUPPORTED_COMMANDS.to_vec();
    }

    fn search_command(query: &str) -> Result<Vec<String>, Error> {
        let path_env = env::var("PATH").unwrap_or_default();

        return Ok(env::split_paths(&path_env)
            .filter_map(|dir| dir.read_dir().ok())
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.starts_with(query))
            .collect::<Vec<_>>());
    }
}
