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
                Token::Value(path) | Token::String(path, _) => {
                    match services.history_handler.load_from(path.into()) {
                        Ok(_) => return Ok(String::new()),
                        Err(err) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Error: {}", err.to_string()),
                            ))
                        }
                    }
                }
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid data")),
            }
        } else if tokens.contains(&Token::Argument("w".to_string(), true))
            || tokens.contains(&Token::Argument("w".to_string(), false))
        {
            let last_token = tokens.iter().last().unwrap();

            match last_token {
                Token::Value(path) | Token::String(path, _) => {
                    match services.history_handler.save_to(path.into()) {
                        Ok(_) => return Ok(String::new()),
                        Err(err) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Error: {}", err.to_string()),
                            ))
                        }
                    }
                }
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid data")),
            }
        } else if tokens.contains(&Token::Argument("a".to_string(), true))
            || tokens.contains(&Token::Argument("a".to_string(), false))
        {
            let last_token = tokens.iter().last().unwrap();

            match last_token {
                Token::Value(path) | Token::String(path, _) => {
                    match services.history_handler.append_to(path.into()) {
                        Ok(_) => return Ok(String::new()),
                        Err(err) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Error: {}", err.to_string()),
                            ))
                        }
                    }
                }
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid data")),
            }
        } else if tokens.len() > 2 {
            let last_token = tokens.iter().last().unwrap();

            match last_token {
                Token::Value(count_str) if let Ok(count) = count_str.parse::<usize>() => {
                    let history = services.history_handler.get_all();
                    let len = &history.len();
                    let last_n = history
                        .iter()
                        .enumerate()
                        .skip(len - count as usize)
                        .map(|(i, h)| format!("{} {}", i + 1, h))
                        .collect::<Vec<String>>()
                        .join("\n");

                    return Ok(last_n);
                }
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
