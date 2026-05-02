use shell_starter_rust::{
    interpreter::Interpreter,
    provider::CommandProvider,
    shell::Shell,
    tokenizer::{Token, Tokenizer},
};

#[tokio::main]
async fn main() {
    let mut shell = Shell::new();

    let _ = shell
        .run::<Token, Interpreter, CommandProvider, Tokenizer>()
        .await;
}
