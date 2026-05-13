use std::io::{Error, ErrorKind};

use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::{ShellCommand, ShellHistoryHandler},
};

pub struct History {}

impl ShellCommand<Token, ServiceContainer> for History {
    fn run(tokens: &[Token], services: &mut ServiceContainer) -> Result<String, std::io::Error> {
        if tokens.contains(&Token::Argument("r".to_string(), true))
            || tokens.contains(&Token::Argument("r".to_string(), false))
        {
            let last_token = tokens.iter().last().unwrap();

            match last_token {
                Token::Value(path) | Token::String(path, _)
                    if services.history_handler.load_from_path(path.into()).is_ok() =>
                {
                    return Ok(String::new());
                }
                Token::String(_, _) => todo!(),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Argument is invalid")),
            }
        }

        let history = services.history_handler.get_all();
        let output = history
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{} {}", i + 1, h))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(output)
    }
}
