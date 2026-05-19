#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Space,
    Value(String),
    Argument(String, bool),
    String(String, bool),
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
        }
    }

    pub fn is_redirection_token(&self) -> bool {
        self.is_append() || self.is_redirect()
    }

    pub fn is_redirect(&self) -> bool {
        match self {
            Token::Value(val) if val == ">" || val == "1>" || val == "2>" => true,
            _ => false,
        }
    }

    pub fn is_append(&self) -> bool {
        match self {
            Token::Value(val) if val == ">>" || val == "1>>" || val == "2>>" => true,
            _ => false,
        }
    }

    pub fn is_redirecting_ok(&self) -> bool {
        match self {
            Token::Value(val) if val == ">" || val == "1>" ||  val == ">>"  || val == "1>>"=> true,
            _ => false,
        }
    }

    pub fn is_redirecting_err(&self) -> bool {
        match self {
            Token::Value(val) if val == "2>" || val == "2>>" => true,
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
            Self::Space => Self::Space,
            Self::Value(arg0) => Self::Value(arg0.clone()),
            Self::Argument(arg0, arg1) => Self::Argument(arg0.clone(), arg1.clone()),
            Self::String(arg0, arg1) => Self::String(arg0.clone(), arg1.clone()),
        }
    }
}
