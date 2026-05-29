use crate::modules::tokenizer::helpers::{Argument, Redirectable};

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Value(String),
    String(String, bool),
}

impl Token {
    pub fn serialize(&self) -> String {
        match self {
            Token::Value(val) => val.to_string(),
            Token::String(val, _) => val.to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.serialize().is_empty()
    }
}

impl Redirectable for Token {
    fn is_redirect(&self) -> bool {
        match self {
            Token::Value(val) if val == ">" || val == "1>" || val == "2>" => true,
            _ => false,
        }
    }

    fn is_append(&self) -> bool {
        match self {
            Token::Value(val) if val == ">>" || val == "1>>" || val == "2>>" => true,
            _ => false,
        }
    }

    fn is_redirecting_ok(&self) -> bool {
        match self {
            Token::Value(val) if val == ">" || val == "1>" || val == ">>" || val == "1>>" => true,
            _ => false,
        }
    }

    fn is_redirecting_err(&self) -> bool {
        match self {
            Token::Value(val) if val == "2>" || val == "2>>" => true,
            _ => false,
        }
    }
}

impl Argument for Token {
    fn is_arg(&self) -> bool {
        match self {
            Token::Value(token) => token.starts_with('-'),
            _ => false,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Value(arg0) => Self::Value(arg0.clone()),
            Self::String(arg0, arg1) => Self::String(arg0.clone(), arg1.clone()),
        }
    }
}
