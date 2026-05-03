use std::{
    fmt::Display,
    io::{Error, ErrorKind},
};

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Space,
    Value(String),
    Argument(String, bool),
    String(String, bool),
    Redirector(char),
    Appender(char),
}

impl Token {
    pub fn serialize(&self) -> String {
        match self {
            Token::Space => String::from(" "),
            Token::Value(val) => val.to_string(),
            Token::Argument(val, is_double) => {
                let dashes = if *is_double { "--" } else { "-" };
                format!("{}{}", dashes, val.to_string())
            }
            Token::String(val, _) => val.to_string(),
            Token::Redirector(num) => format!("{}>", num),
            Token::Appender(num) => format!("{}>>", num),
        }
    }

    pub fn is_redirection_token(&self) -> bool {
        match self {
            Token::Space => false,
            Token::Value(_) => false,
            Token::Argument(_, _) => false,
            Token::String(_, _) => false,
            Token::Redirector(_) => true,
            Token::Appender(_) => true,
        }
    }

    pub fn is_redirection_ok(&self) -> Result<bool, Error> {
        match self {
            Token::Space => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Value(_) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Argument(_, _) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::String(_, _) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Redirector(prefix) => Ok(prefix == &'1'),
            Token::Appender(prefix) => Ok(prefix == &'1'),
        }
    }

    pub fn is_redirection_err(&self) -> Result<bool, Error> {
        match self {
            Token::Space => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Value(_) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Argument(_, _) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::String(_, _) => Err(Error::new(
                ErrorKind::Other,
                "Checked token is not a redirection token",
            )),
            Token::Redirector(prefix) => Ok(prefix == &'2'),
            Token::Appender(prefix) => Ok(prefix == &'2'),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Space => Self::Space,
            Self::Value(arg0) => Self::Value(arg0.clone()),
            Self::Argument(arg0, arg1) => Self::Argument(arg0.clone(), arg1.clone()),
            Self::String(arg0, arg1) => Self::String(arg0.clone(), arg1.clone()),
            Self::Redirector(arg0) => Self::Redirector(arg0.clone()),
            Self::Appender(arg0) => Self::Appender(arg0.clone()),
        }
    }
}
