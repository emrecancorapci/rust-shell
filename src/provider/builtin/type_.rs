use std::io::{Error, ErrorKind};

use crate::{
    provider::SUPPORTED_COMMANDS, shell::core::ShellCommand, tokenizer::Token,
    util::path::ExecutionPath,
};

pub struct Type {}

impl ShellCommand<Token> for Type {
    fn run(tokens: &[Token]) -> Result<String, Error> {
        if tokens.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "This command needs argument.",
            ));
        }

        for command in SUPPORTED_COMMANDS.iter() {
            if tokens.get(2) == Some(&Token::Value(command.to_string())) {
                return Ok(format!("{} is a shell builtin", command));
            }
        }

        match tokens.get(2).unwrap() {
            Token::Space => {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Third token shouldn't be a space. Fix this.",
                ))
            }
            Token::Value(input) | Token::String(input, _) => match input.get_exec_path() {
                Some(path) => return Ok(format!("{} is {}", input, path.to_str().unwrap())),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} not found", input),
                    ))
                }
            },
            Token::Argument(_, _) => todo!(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "bash: syntax error near unexpected token 'newline'",
                ))
            }
        }
    }
}
