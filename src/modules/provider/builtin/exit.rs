use std::io::{Error, ErrorKind};

use crate::{shell::core::ShellCommand, modules::tokenizer::Token};

pub struct Exit {}

impl ShellCommand<Token> for Exit {
    fn run(tokens: &[Token]) -> Result<String, Error> {
        if tokens.get(2) == Some(&Token::Value("0".to_string())) {
            return Err(Error::new(ErrorKind::Interrupted, ""));
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{}: command not found", tokens[0]),
            ));
        }
    }
}
