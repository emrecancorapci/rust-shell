use std::io::{Error, ErrorKind};

use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::ShellCommand,
};
pub struct Echo {}

impl ShellCommand<Token, ServiceContainer> for Echo {
    fn run(tokens: &[Token], _services: &mut ServiceContainer) -> Result<String, std::io::Error> {
        if tokens.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No string input found.",
            ));
        }

        return Ok(tokens
            .iter()
            .skip(2)
            .fold(String::new(), |a, b| a + &b.serialize()));
    }
}
