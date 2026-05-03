use std::{
    io::{Error, ErrorKind},
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub use token::Token;

use crate::shell::core::ShellTokenizer;

mod token;

#[derive(PartialEq, Eq)]
enum ParseMode {
    None,
    Value,
    SingleQuote,
    DoubleQuote,
    SingleDashArg,
    DoubleDashArg,
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
                    '\\' => {
                        mode = ParseMode::Value;

                        match iter.peek() {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                buffer.push(ch)
                            }
                            None => todo!(),
                        }
                    }
                    '-' => {
                        if matches!(iter.peek(), Some(&(_, '-'))) {
                            iter.next();
                            mode = ParseMode::DoubleDashArg
                        } else {
                            mode = ParseMode::SingleDashArg
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' | '.' | '/' | '~' if buffer.is_empty() => {
                        mode = ParseMode::Value;
                        buffer.push(ch);
                    }
                    '0'..='9' if buffer.is_empty() => {
                        if let Some((_, '>')) = iter.peek() {
                            iter.next();
                            tokens.push(parse_redirector(&mut iter, ch)?)
                        } else {
                            buffer.push(ch);
                            mode = ParseMode::Value;
                        }
                    }
                    '>' => tokens.push(parse_redirector(&mut iter, '1')?),
                    ' ' => {
                        if tokens.last() != Some(&Token::Space) {
                            tokens.push(Token::Space)
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::Value => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | '/' => buffer.push(ch),
                    '\\' => {
                        let ch = iter.peek();

                        match ch {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                buffer.push(ch)
                            }
                            None => todo!(),
                        }
                    }
                    ' ' => {
                        tokens.push(generate_token(mode, &buffer));
                        tokens.push(Token::Space);

                        buffer = String::new();
                        mode = ParseMode::None;
                        sub_mode = ParseMode::None;
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
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
                ParseMode::SingleDashArg | ParseMode::DoubleDashArg => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => buffer.push(ch),
                    ' ' => {
                        tokens.push(generate_token(mode, &buffer));
                        tokens.push(Token::Space);

                        buffer = String::new();
                        mode = ParseMode::None;
                        sub_mode = ParseMode::None;
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
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

fn parse_redirector(
    iter: &mut Peekable<Enumerate<Chars<'_>>>,
    prefix: char,
) -> Result<Token, Error> {
    match iter.peek() {
        Some((_, '>')) => {
            iter.next();
            Ok(Token::Appender(prefix))
        }
        Some(_) => Ok(Token::Redirector(prefix)),
        None => return Err(Error::new(ErrorKind::InvalidInput, "No redirection target")),
    }
}

fn generate_token(mode: ParseMode, value: &str) -> Token {
    match mode {
        ParseMode::None => panic!("Tried to push a token before it started to parse anything"),
        ParseMode::Value => Token::Value(value.to_string()),
        ParseMode::SingleQuote => Token::String(value.to_string(), false),
        ParseMode::DoubleQuote => Token::String(value.to_string(), true),
        ParseMode::SingleDashArg => Token::Argument(value.to_string(), false),
        ParseMode::DoubleDashArg => Token::Argument(value.to_string(), true),
    }
}
