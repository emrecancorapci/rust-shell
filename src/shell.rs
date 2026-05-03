use std::io::{self, Error, ErrorKind, Stderr, Stdout, Write};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub mod core;

pub struct Shell {
    pub(crate) buffer: String,
    pub(crate) stdout: Stdout,
    pub(crate) stderr: Stderr,

    // Auto Complete
    pub(crate) tab_query: String,
    pub(crate) tab_index: u8,
    // history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stderr: io::stderr(),
            tab_index: 0,
            tab_query: String::new(),
            // history: Vec::new(),
        }
    }

    pub async fn run<
        T,
        I: ShellInterpreter<T>,
        C: ShellCommandProvider<T>,
        K: ShellTokenizer<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        self.init()?;

        loop {
            self.stdout.flush()?;

            let result = self.handle_event::<T, I, C, K>();

            if result.is_err() && result.unwrap_err().kind() == ErrorKind::Interrupted {
                break;
            }
        }

        self.uninit()?;

        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;

        execute!(self.stdout, Print("$ "),)?;
        Ok(())
    }

    fn uninit(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;

        execute!(self.stdout)?;
        Ok(())
    }
}
