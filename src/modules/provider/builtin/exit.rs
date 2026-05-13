use std::io::{Error, ErrorKind};

use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::ShellCommand,
};

pub struct Exit {}

impl ShellCommand<Token, ServiceContainer> for Exit {
    fn run(tokens: &[Token], _services: &mut ServiceContainer) -> Result<String, Error> {
        if tokens.len() > 2 && tokens.get(2) == Some(&Token::Value("0".to_string())) {
            return Err(Error::new(ErrorKind::Interrupted, ""));
        } else if tokens.len() == 1 {
            return Err(Error::new(ErrorKind::Interrupted, ""));
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{}: command not found", tokens[0]),
            ));
        }
    }
}
