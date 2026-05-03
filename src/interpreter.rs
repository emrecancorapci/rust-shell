use std::{
    fs,
    io::{Error, ErrorKind},
    process::Output,
};

use crate::{
    shell::core::{ShellCommandProvider, ShellInterpreter},
    tokenizer::Token,
    util::{error::AsBytes, output::SplitOutput, path::ExecutionPath},
};

pub struct Interpreter {}

impl ShellInterpreter<Token> for Interpreter {
    fn run<CP: ShellCommandProvider<Token>>(tokens: &[Token]) -> Result<Vec<u8>, Error> {
        match tokens.iter().any(|t| t.is_redirection_token()) {
            true => Self::handle_redirected_input::<CP>(tokens),
            false => Self::handle_direct_input::<CP>(tokens),
        }
    }
}

impl Interpreter {
    fn handle_direct_input<CP: ShellCommandProvider<Token>>(
        tokens: &[Token],
    ) -> Result<Vec<u8>, Error> {
        let cmd_token = tokens.iter().find(|token| match token {
            Token::Value(_) => true,
            _ => false,
        });

        if cmd_token.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "error: no command provided"))
        }

        match cmd_token.unwrap() {
            Token::Value(cmd) | Token::String(cmd, _) if cmd.get_exec_path().is_some() => {
                let output = Self::execute_external(tokens, cmd)?;

                if output.status.success() {
                    let mut output_array = output.stdout.to_vec();

                    if output_array.last() == Some(&10) {
                        output_array.pop();
                    }

                    return Ok(output_array);
                }

                let mut error_array = output.stderr.to_vec();

                if error_array.last() == Some(&10) {
                    error_array.pop();
                }

                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    String::from_utf8(error_array).unwrap(),
                ));
            }
            Token::Value(cmd) | Token::String(cmd, _) => match CP::run(cmd, tokens) {
                Ok(response) => return Ok(response.as_bytes().to_vec()),
                Err(err) => return Err(err),
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
        }
    }

    fn execute_external(tokens: &[Token], cmd: &String) -> Result<Output, Error> {
        let input_array = tokens
            .iter()
            .skip(2)
            .filter(|i| !matches!(i, Token::Space))
            .map(|i| i.serialize());

        std::process::Command::new(cmd).args(input_array).output()
    }

    fn handle_redirected_input<CP: ShellCommandProvider<Token>>(
        tokens: &[Token],
    ) -> Result<Vec<u8>, Error> {
        let redirection_index = tokens
            .iter()
            .position(|t| t.is_redirection_token())
            .unwrap();

        let (tokens, redirection_tokens) = tokens.split_at(redirection_index);

        let (response, error) = match tokens.first() {
            Some(Token::Value(cmd) | Token::String(cmd, _)) if cmd.get_exec_path().is_some() => {
                let output = Self::execute_external(tokens, cmd)?;

                output.split_output()
            }
            Some(Token::Value(cmd) | Token::String(cmd, _)) => {
                match CP::run(cmd, &tokens.to_vec()) {
                    Ok(response) => (Some(response.as_bytes().to_vec()), None),
                    Err(err) => (None, Some(err)),
                }
            }
            Some(_) => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
            None => return Ok(vec![]),
        };

        Self::execute_redirected(redirection_tokens, response, error)
    }

    fn execute_redirected(
        redirection_tokens: &[Token],
        output: Option<Vec<u8>>,
        error: Option<Error>,
    ) -> Result<Vec<u8>, Error> {
        let path = redirection_tokens.get(2).unwrap().serialize();

        match redirection_tokens.first().unwrap() {
            Token::Redirector('1') => {
                fs::write(path, output.unwrap())?;

                match error {
                    Some(err) => Err(err),
                    None => Ok(vec![]),
                }
            }
            Token::Redirector('2') => {
                fs::write(path, error.unwrap().to_string().as_bytes())?;

                match output {
                    Some(output) => Ok(output),
                    None => Ok(vec![]),
                }
            }
            Token::Appender('1') => {
                Self::append_to_file(&path, &output.unwrap())?;

                match error {
                    Some(err) => Err(err),
                    None => Ok(vec![]),
                }
            }
            Token::Appender('2') => {
                Self::append_to_file(&path, &error.unwrap().as_bytes())?;

                match output {
                    Some(output) => Ok(output),
                    None => Ok(vec![]),
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "error: invalid redirection",
                ))
            }
        }
    }

    fn append_to_file(path: &str, content: &[u8]) -> Result<(), Error> {
        let mut contents = fs::read(&path)?;

        if !contents.is_empty() {
            contents.extend_from_slice(&[10]);
        }

        contents.extend_from_slice(content);
        fs::write(path, contents)?;

        Ok(())
    }
}
