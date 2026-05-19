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
        if tokens.len() < 3 {
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
            Token::Space => {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Third token shouldn't be a space. Fix this.",
                ))
            }
            Token::Value(input) | Token::String(input, _) => match input.get_exec_path() {
                Some(path) => {
                    return Ok(format!("{} is {}", input, path.to_str().unwrap()));
                }
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} not found", input),
                    ))
                }
            },
            Token::Argument(_, _) => todo!(),
        }
    }
}
