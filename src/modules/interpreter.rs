use std::{
    fs,
    io::{Error, ErrorKind},
    process::Output,
};

use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::{ShellCommandProvider, ShellInterpreter},
    util::{error::AsBytes, output::SplitOutput, path::ExecutionPath},
};

pub struct Interpreter {}

impl ShellInterpreter<Token, ServiceContainer> for Interpreter {
    fn run<CommandProvider: ShellCommandProvider<Token, ServiceContainer>>(
        tokens: &[Token],
        services: &ServiceContainer,
    ) -> Result<Vec<u8>, Error> {
        match tokens.iter().any(|t| t.is_redirection_token()) {
            true => Self::handle_redirected_input::<CommandProvider>(tokens, services),
            false => Self::handle_direct_input::<CommandProvider>(tokens, services),
        }
    }
}

impl Interpreter {
    fn handle_direct_input<CommandProvider: ShellCommandProvider<Token, ServiceContainer>>(
        tokens: &[Token],
        services: &ServiceContainer,
    ) -> Result<Vec<u8>, Error> {
        let cmd_token = tokens.iter().find(|token| match token {
            Token::Value(_) | Token::String(_, _) => true,
            _ => false,
        });

        if cmd_token.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "error: no command provided",
            ));
        }

        match cmd_token.unwrap() {
            Token::Value(cmd) | Token::String(cmd, _) => {
                match CommandProvider::run(cmd, tokens, services) {
                    Ok(response) => return Ok(response.as_bytes().to_vec()),
                    Err(err)
                        if err.kind() == ErrorKind::NotFound && cmd.get_exec_path().is_some() =>
                    {
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
                    Err(err) => return Err(err),
                }
            }
            _ => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
        }
    }

    fn execute_external(tokens: &[Token], cmd: &String) -> Result<Output, Error> {
        let args: Vec<String> = tokens
            .iter()
            .skip(2)
            .fold(Vec::<Vec<&Token>>::new(), |mut groups, token| {
                if matches!(token, Token::Space) {
                    // A space starts a new group
                    groups.push(Vec::new());
                } else {
                    // Append to the last group, or create first group
                    if groups.is_empty() {
                        groups.push(Vec::new());
                    }
                    groups.last_mut().unwrap().push(token);
                }
                groups
            })
            .into_iter()
            .filter(|g| !g.is_empty())
            .map(|g| g.iter().map(|t| t.serialize()).collect::<String>())
            .collect();

        std::process::Command::new(cmd).args(args).output()
    }

    fn handle_redirected_input<CommandProvider: ShellCommandProvider<Token, ServiceContainer>>(
        tokens: &[Token],
        services: &ServiceContainer,
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
                match CommandProvider::run(cmd, &tokens.to_vec(), services) {
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
                if output.is_none() {
                    return match error {
                        Some(err) => Err(err),
                        None => Ok(vec![]),
                    };
                }

                fs::write(path, output.unwrap())?;

                match error {
                    Some(err) => Err(err),
                    None => Ok(vec![]),
                }
            }
            Token::Redirector('2') => {
                fs::write(
                    path,
                    &error
                        .unwrap_or_else(|| Error::new(ErrorKind::Other, ""))
                        .to_string()
                        .as_bytes(),
                )?;

                match output {
                    Some(output) => Ok(output),
                    None => Ok(vec![]),
                }
            }
            Token::Appender('1') => {
                if output.is_none() {
                    Self::append_to_file(&path, b"")?;

                    return match error {
                        Some(err) => Err(err),
                        None => Ok(vec![]),
                    };
                }

                Self::append_to_file(&path, &output.unwrap())?;

                match error {
                    Some(err) => Err(err),
                    None => Ok(vec![]),
                }
            }
            Token::Appender('2') => {
                if error.is_none() {
                    return match output {
                        Some(output) => Ok(output),
                        None => Ok(vec![]),
                    };
                }

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
        let contents = fs::read(&path);

        if contents.is_err() {
            fs::write(path, &content)?;

            return Ok(());
        }

        let mut contents = contents.unwrap();

        if !contents.is_empty() {
            contents.extend_from_slice(&[10]);
        }

        contents.extend_from_slice(content);
        fs::write(path, contents)?;

        Ok(())
    }
}
