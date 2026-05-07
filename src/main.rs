use shell_starter_rust::{
    modules::{
        auto_complete::AutoComplete,
        interpreter::Interpreter,
        provider::CommandProvider,
        tokenizer::{Token, Tokenizer},
    },
    shell::Shell,
};

#[tokio::main]
async fn main() {
    let mut shell = Shell::new();

    let _ = shell
        .run::<Token, Interpreter, CommandProvider, Tokenizer, AutoComplete>()
        .await;
}
