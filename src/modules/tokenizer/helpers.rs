use crate::modules::tokenizer::Token;

pub trait HasArguments {
    fn contains_arg(&self, arg: &str) -> bool;
    fn get_arg_value(&self, arg: &str) -> Option<String>;
}

impl HasArguments for &[Token] {
    fn contains_arg(&self, arg: &str) -> bool {
        let m1 = format!("--{}", arg);
        let m2 = format!("-{}", arg);

        self.iter().any(|token| match token {
            Token::Value(val) => val == &m1 || val == &m2,
            _ => false,
        })
    }

    fn get_arg_value(&self, arg: &str) -> Option<String> {
        let mut iter = self.iter().peekable();

        while let Some(token) = iter.next() {
            if token.serialize() == arg {
                return iter.peek().map(|next_token| next_token.serialize());
            }
        }

        None
    }
}

pub trait Argument {
    fn is_arg(&self) -> bool;
}

pub trait Redirectable {
    fn is_redirector(&self) -> bool {
        self.is_append() || self.is_redirect()
    }
    fn is_redirect(&self) -> bool;
    fn is_append(&self) -> bool;
    fn is_redirecting_ok(&self) -> bool;
    fn is_redirecting_err(&self) -> bool;
}

impl Argument for String {
    fn is_arg(&self) -> bool {
        self.starts_with('-')
    }
}

impl Redirectable for String {
    fn is_redirect(&self) -> bool {
        (self.starts_with(">") || self.starts_with("1>") || self.starts_with("2>"))
            && !self.is_append()
    }

    fn is_append(&self) -> bool {
        self.starts_with(">>") || self.starts_with("1>>") || self.starts_with("2>>")
    }

    fn is_redirecting_ok(&self) -> bool {
        self.starts_with(">") || self.starts_with("1>")
    }

    fn is_redirecting_err(&self) -> bool {
        self.starts_with("2>")
    }
}
