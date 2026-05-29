use std::io::{Error, ErrorKind};

use crate::{
    modules::{
        service_container::ServiceContainer,
        tokenizer::{
            helpers::{HasArguments, Argument, Redirectable},
            Token,
        },
    },
    shell::core::{ShellCommand, ShellHistoryHandler},
};

pub struct History {}

impl ShellCommand<Token, ServiceContainer> for History {
    fn run(tokens: &[Token], services: &mut ServiceContainer) -> Result<String, std::io::Error> {
        if let Some(path) = tokens.get_arg_value("r") {
            if !path.is_arg() && !path.is_redirect() {
                match services.history_handler.load_from(path.into()) {
                    Ok(_) => return Ok(String::new()),
                    Err(err) => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Error: {}", err.to_string()),
                        ))
                    }
                }
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid data"));
            }
        } else if let Some(path) = tokens.get_arg_value("w") {
            if !path.is_arg() && !path.is_redirect() {
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
        }
        if let Some(path) = tokens.get_arg_value("r") {
            if !path.is_arg() && !path.is_redirect() {
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
