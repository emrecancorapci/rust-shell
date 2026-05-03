use std::io::{self, Error, ErrorKind, Stderr, Stdout, Write};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    event::{self, Event},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub mod core;
pub mod key_handler;

pub struct Shell {
    buffer: String,
    stdout: Stdout,
    stderr: Stderr,

    // Auto Complete
    tab_query: String,
    tab_index: u8,
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

            let result = self.shell_loop::<T, I, C, K>();

            if result.is_err() && result.unwrap_err().kind() == ErrorKind::Interrupted {
                break;
            }
        }

        self.uninit()?;

        Ok(())
    }

    fn shell_loop<T, I: ShellInterpreter<T>, C: ShellCommandProvider<T>, K: ShellTokenizer<T>>(
        &mut self,
    ) -> Result<(), Error> {
        match event::read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key_event) => self.handle_keys::<T, I, C, K>(key_event),
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
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
