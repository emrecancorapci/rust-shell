use std::io::{self, Error, ErrorKind, Stderr, Stdout, Write};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{modules::auto_complete::AutoComplete, shell::core::ShellAutoComplete};

pub mod core;

pub struct Shell {
    pub(crate) buffer: String,
    pub(crate) stdout: Stdout,
    pub(crate) stderr: Stderr,

    pub(crate) cursor_x: u16,
    pub(crate) cursor_y: u16,

    // Auto Complete
    pub(crate) auto_complete: AutoComplete,
    // history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stderr: io::stderr(),
            // history: Vec::new(),
            auto_complete: AutoComplete::new(),
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub async fn run<
        T,
        I: ShellInterpreter<T>,
        C: ShellCommandProvider<T>,
        K: ShellTokenizer<T>,
        A: ShellAutoComplete,
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
        self.cursor_x = 2;
        Ok(())
    }

    fn uninit(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;

        execute!(self.stdout)?;
        Ok(())
    }
}
