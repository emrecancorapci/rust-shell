use std::{collections::HashSet, env};

use crate::{
    modules::tokenizer::Token,
    shell::core::{ShellCommand, ShellCommandProvider},
};

use builtin::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, type_::Type};

pub mod builtin;

pub const SUPPORTED_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

pub struct CommandProvider {}

impl ShellCommandProvider<Token> for CommandProvider {
    fn run(cmd: &str, tokens: &[Token]) -> Result<String, std::io::Error> {
        match cmd {
            "echo" => Echo::run(tokens),
            "type" => Type::run(tokens),
            "exit" => Exit::run(tokens),
            "pwd" => Pwd::run(tokens),
            "cd" => Cd::run(tokens),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "shell: command not found",
            )),
        }
    }

    fn get_commands() -> Vec<&'static str> {
        return SUPPORTED_COMMANDS.to_vec();
    }

    fn search_command(query: &str) -> Result<Vec<String>, std::io::Error> {
        let path_env = env::var("PATH").unwrap_or_default();

        return Ok(env::split_paths(&path_env)
            .filter_map(|dir| dir.read_dir().ok())
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.starts_with(query))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>());
    }
}
