use std::{
    fmt::Debug,
    io::{self, Error, ErrorKind, Stderr, Stdout, Write},
};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::modules::service_container::ServiceContainer;

pub mod core;

pub struct Shell {
    pub(crate) buffer: String,
    pub(crate) stdout: Stdout,
    pub(crate) stderr: Stderr,

    pub(crate) cursor_x: u16,
    pub(crate) cursor_y: u16,

    pub(crate) services: ServiceContainer,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stderr: io::stderr(),
            cursor_x: 0,
            cursor_y: 0,
            services: ServiceContainer::new(),
        }
    }

    pub async fn run<
        Token: Debug,
        Interpreter: ShellInterpreter<Token, ServiceContainer>,
        CommandProvider: ShellCommandProvider<Token, ServiceContainer>,
        Tokenizer: ShellTokenizer<Token>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        self.init()?;

        loop {
            self.stdout.flush()?;

            let result = self.handle_event::<Token, Interpreter, CommandProvider, Tokenizer>();

            if result.is_err() && result.unwrap_err().kind() == ErrorKind::Interrupted {
                break;
            }
        }

        self.uninit()?;

        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.services.init()?;

        execute!(self.stdout, Print("$ "),)?;
        self.cursor_x = 2;
        Ok(())
    }

    fn uninit(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        self.services.cleanup()?;

        execute!(self.stdout)?;
        Ok(())
    }
}
