use crate::{
    modules::{auto_complete::AutoComplete, history_handler::HistoryHandler},
    shell::core::{ShellAutoComplete, ShellHistoryHandler},
};

pub struct ServiceContainer {
    pub history_handler: HistoryHandler,
    pub auto_complete: AutoComplete,
}

impl ServiceContainer {
    pub(crate) fn new() -> Self {
        Self {
            history_handler: HistoryHandler::new(),
            auto_complete: AutoComplete::new(),
        }
    }

    pub(crate) fn init(&mut self) -> Result<(), std::io::Error> {
        self.history_handler.load()?;
        Ok(())
    }

    pub(crate) fn cleanup(&mut self) -> Result<(), std::io::Error> {
        self.history_handler.save()?;
        Ok(())
    }
}
