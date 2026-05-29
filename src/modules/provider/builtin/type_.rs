use std::io::{Error, ErrorKind};

use crate::{
    modules::{
        provider::SUPPORTED_COMMANDS, service_container::ServiceContainer, tokenizer::Token,
    },
    shell::core::ShellCommand,
    util::path::ExecutionPath,
};

pub struct Type {}

impl ShellCommand<Token, ServiceContainer> for Type {
    fn run(tokens: &[Token], _services: &mut ServiceContainer) -> Result<String, Error> {
        if tokens.len() < 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "This command needs argument.",
            ));
        }

        // TODO[BUG]: Looks for second index. It should be the first value token after the command.
        for command in SUPPORTED_COMMANDS.iter() {
            if tokens.get(2) == Some(&Token::Value(command.to_string())) {
                return Ok(format!("{} is a shell builtin", command));
            }
        }

        // TODO[BUG]: Also happens here.
        match tokens.get(2).unwrap() {
            Token::Value(input) | Token::String(input, _)
                if let Some(path) = input.get_exec_path() =>
            {
                Ok(format!("{} is {}", input, path.to_str().unwrap()))
            }
            Token::Value(input) | Token::String(input, _) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{} not found", input),
                ))
            }
        }
    }
}
