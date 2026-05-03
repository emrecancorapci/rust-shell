#![warn(rust_2018_idioms, unreachable_pub, clippy::all)]
#![allow(clippy::needless_return)]
#![forbid(unsafe_code)]

pub mod util {
    pub mod error;
    pub mod output;
    pub mod path;
}
pub mod modules {
    pub mod interpreter;
    pub mod tokenizer;
    pub mod provider;
}
pub mod shell;
