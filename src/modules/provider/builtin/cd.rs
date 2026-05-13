use std::{
    env,
    io::{Error, ErrorKind},
};

use crate::{modules::{service_container::ServiceContainer, tokenizer::Token}, shell::core::ShellCommand};
pub struct Cd {}

impl ShellCommand<Token, ServiceContainer> for Cd {
    fn run(tokens: &[Token], _services: &ServiceContainer) -> Result<String, Error> {
        if tokens.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "cd: missing argument"));
        }

        match tokens.get(2) {
            Some(Token::Value(path)) => {
                match path.as_str() {
                    "~" => {
                        // Doesn't work on windows. There is a crate called homedir that can be used to get the home directory.
                        if env::var("HOME").is_err() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("HOME not set"),
                            ));
                        }

                        if env::set_current_dir(env::var("HOME").unwrap()).is_err() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("{}: No such file or directory", env::var("HOME").unwrap()),
                            ));
                        }
                        return Ok(String::new());
                    }
                    path => {
                        if env::set_current_dir(&path).is_err() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("{}: No such file or directory", &path),
                            ));
                        }

                        return Ok(String::new());
                    }
                }
            }
            _ => return Err(Error::new(ErrorKind::InvalidInput, "cd: invalid input")),
        }
    }
}
