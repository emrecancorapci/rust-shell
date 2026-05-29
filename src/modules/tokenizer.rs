use std::io::{Error, ErrorKind};

pub use token::Token;

use crate::shell::core::ShellTokenizer;

mod token;
pub mod helpers;

#[derive(PartialEq, Eq, Debug)]
enum ParseMode {
    None,
    Value,
    SingleQuote,
    DoubleQuote,
}

pub struct Tokenizer {}

impl ShellTokenizer<Token> for Tokenizer {
    fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
        let mut iter = input.chars().into_iter().enumerate().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer = String::new();
        let mut mode = ParseMode::None;
        let mut sub_mode = ParseMode::None;

        while let Some((i, ch)) = iter.next() {
            match mode {
                ParseMode::None => match ch {
                    '\'' => mode = ParseMode::SingleQuote,
                    '"' => mode = ParseMode::DoubleQuote,
                    '\\' if let Some((_i, val)) = iter.peek() => {
                        buffer.push(*val);
                        mode = ParseMode::Value;

                        iter.next();
                    }
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '/' | '~' | '>' | '-'
                        if buffer.is_empty() =>
                    {
                        mode = ParseMode::Value;
                        buffer.push(ch);
                    }
                    ' ' => {
                        if tokens.last() != Some(&Token::Space) {
                            tokens.push(Token::Space)
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ));
                    }
                },
                ParseMode::Value => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | '/' | ':' | '>' => {
                        buffer.push(ch)
                    }
                    '\\' if let Some((_i, val)) = iter.peek() => {
                        buffer.push(*val);
                        iter.next();
                    }
                    '"' => {
                        tokens.push(generate_token(mode, &buffer));

                        mode = ParseMode::DoubleQuote;
                        sub_mode = ParseMode::None;
                        buffer = String::new();
                    }
                    '\'' => {
                        tokens.push(generate_token(mode, &buffer));

                        mode = ParseMode::SingleQuote;
                        sub_mode = ParseMode::None;
                        buffer = String::new();
                    }
                    ' ' => {
                        tokens.push(generate_token(mode, &buffer));
                        tokens.push(Token::Space);

                        mode = ParseMode::None;
                        sub_mode = ParseMode::None;
                        buffer = String::new();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ));
                    }
                },
                ParseMode::SingleQuote => match ch {
                    '\'' => {
                        tokens.push(generate_token(mode, &buffer));

                        buffer = String::new();
                        mode = ParseMode::None;
                        sub_mode = ParseMode::None;
                    }
                    _ => buffer.push(ch),
                },
                ParseMode::DoubleQuote => match ch {
                    '"' => {
                        tokens.push(generate_token(mode, &buffer));

                        buffer = String::new();
                        mode = ParseMode::None;
                        sub_mode = ParseMode::None;
                    }
                    '\\' => {
                        if sub_mode == ParseMode::SingleQuote {
                            buffer.push(ch);
                            continue;
                        }

                        match iter.peek() {
                            Some((_, '\\' | '$' | '"')) => {
                                let (_index, ch) = iter.next().unwrap();

                                buffer.push(ch)
                            }
                            Some(_) => {
                                buffer.push(ch);
                            }
                            None => todo!(),
                        }
                    }
                    '\'' => match sub_mode {
                        ParseMode::None => {
                            buffer.push(ch);
                            sub_mode = ParseMode::SingleQuote;
                        }
                        ParseMode::SingleQuote => {
                            buffer.push(ch);
                            sub_mode = ParseMode::None;
                        }
                        _ => todo!(),
                    },
                    _ => buffer.push(ch),
                },
            }
        }

        match mode {
            ParseMode::SingleQuote => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Single quote didn't end.",
                ))
            }
            ParseMode::DoubleQuote => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Double quote didn't end.",
                ))
            }
            ParseMode::None => {
                return Ok(tokens.to_vec());
            }
            _ => {
                tokens.push(generate_token(mode, &buffer));
                return Ok(tokens.to_vec());
            }
        }
    }
}

fn generate_token(mode: ParseMode, value: &str) -> Token {
    match mode {
        ParseMode::None => panic!("Tried to push a token before it started to parse anything"),
        ParseMode::Value => Token::Value(value.trim().to_string()),
        ParseMode::SingleQuote => Token::String(value.to_string(), false),
        ParseMode::DoubleQuote => Token::String(value.to_string(), true),
    }
}
