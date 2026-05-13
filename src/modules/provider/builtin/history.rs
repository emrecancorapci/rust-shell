use crate::{
    modules::{service_container::ServiceContainer, tokenizer::Token},
    shell::core::{ShellCommand, ShellHistoryHandler},
};

pub struct History {}

impl ShellCommand<Token, ServiceContainer> for History {
    fn run(tokens: &[Token], services: &ServiceContainer) -> Result<String, std::io::Error> {
        if tokens.len() > 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "history: too many arguments",
            ));
        }

        let history = services.history_handler.get_all();
        let output = history
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{} {}", i + 1, h))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(output)
    }
}
