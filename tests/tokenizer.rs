use std::io::ErrorKind;

use shell_starter_rust::{
    modules::tokenizer::{
        Token::{self, String, Value},
        Tokenizer,
    },
    shell::core::ShellTokenizer,
};

// Basic

#[test]
fn empty_input() {
    let input = "";
    let expected: Vec<Token> = vec![];

    assert_parsing(input, expected);
}

#[test]
fn basic_command() {
    let input = "hello world";
    let expected = vec![Value("hello".to_string()), Value("world".to_string())];

    assert_parsing(input, expected);
}

#[test]
fn multiple_spaces() {
    let input = "hello                 world";
    let expected = vec![Value("hello".to_string()), Value("world".to_string())];

    assert_parsing(input, expected);
}

// Quotes

#[test]
fn single_quote() {
    let input = "echo 'example test'";
    let expected = vec![
        Value("echo".to_string()),
        String("example test".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn unclosed_single_quote() {
    assert_parsing_err("echo 'unclosed single quote");
}

#[test]
fn double_quote() {
    let input = "echo \"hello world\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn unclosed_double_quote() {
    let input = "echo \"unclosed double quote";

    assert_parsing_err(input);
}

#[test]
fn double_quotes_with_escaped_characters() {
    let input = "echo \"escaped \\\"double quotes\\\"\"";
    let expected = vec![
        Value("echo".to_string()),
        String("escaped \"double quotes\"".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_quotes_with_wide_space() {
    let input = "echo \"hello                   world\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello                   world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_quotes_with_wide_space_2() {
    let input = "echo world\\ \\ \\ \\ \\ \\ hello";
    let expected = vec![
        Value("echo".to_string()),
        Value("world      hello".to_string()),
    ];

    assert_parsing(input, expected);
}

#[test]
fn escaped_backslash_in_double_quote() {
    let input = "echo \"hello\\\\world\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello\\world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn escaped_backslash_in_double_quote_2() {
    let input = "echo \\'\\\"example script\\\"\\'";
    let expected = vec![
        Value("echo".to_string()),
        Value("\'\"example".to_string()),
        Value("script\"\'".to_string()),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_inside_single_quote() {
    let input = "echo '\"hello world\"'";
    let expected = vec![
        Value("echo".to_string()),
        String("\"hello world\"".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn single_inside_double_quote() {
    let input = "echo \"'hello world'\"";
    let expected = vec![
        Value("echo".to_string()),
        String("'hello world'".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn mixed_quotes_and_arguments() {
    let input = "cmd 'single' \"double\" --arg1 -a";
    let expected = vec![
        Value("cmd".to_string()),
        String("single".to_string(), false),
        String("double".to_string(), true),
        Value("--arg1".to_string()),
        Value("-a".to_string()),
    ];

    assert_parsing(input, expected);
}

// Arguments

#[test]
fn single_dash_argument() {
    let input = "echo -s 'hello world'";
    let expected = vec![
        Value("echo".to_string()),
        Value("-s".to_string()),
        String("hello world".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_dash_argument() {
    let input = "echo --silent 'hello world'";
    let expected = vec![
        Value("echo".to_string()),
        Value("--silent".to_string()),
        String("hello world".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn redirection_operator() {
    let input = "echo \"hello world\" > \"./hello.md\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello world".to_string(), true),
        Value('>'.to_string()),
        String("./hello.md".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn error_redirection_operator() {
    let input = "echo \"hello world\" 2> \"./hello.md\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello world".to_string(), true),
        Value("2>".to_string()),
        String("./hello.md".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn redirection_without_target() {
    let input = "echo >";
    let expected = vec![Value("echo".to_string()), Value(">".to_string())];

    assert_parsing(input, expected);
}

#[test]
fn appender() {
    let input = "echo \"hello world\" >> \"./hello.md\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello world".to_string(), true),
        Value(">>".to_string()),
        String("./hello.md".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn appender_with_number() {
    let input = "echo \"hello world\" 2>> \"./hello.md\"";
    let expected = vec![
        Value("echo".to_string()),
        String("hello world".to_string(), true),
        Value("2>>".to_string()),
        String("./hello.md".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn invalid_character() {
    let input = "echo hello @world";

    assert_parsing_err(input);
}

#[test]
fn mixed_quotes() {
    let input = "echo \"double quotes\" 'single quotes'";
    let expected = vec![
        Value("echo".to_string()),
        String("double quotes".to_string(), true),
        String("single quotes".to_string(), false),
    ];

    assert_parsing(input, expected);
}

fn assert_vec_eq<T: std::fmt::Debug + PartialEq>(vec1: &[T], vec2: &[T]) {
    if vec1 != vec2 {
        panic!(
            "Vectors are not equal.\nLeft: {:?}\nRight: {:?}",
            vec1, vec2
        );
    }
}

fn assert_parsing(input: &str, expected: Vec<Token>) {
    match Tokenizer::tokenize(input) {
        Ok(tokens) => assert_vec_eq(&tokens, &expected),
        Err(err) => panic!("Unexpected error: {}", err),
    }
}

fn assert_parsing_err(input: &str) {
    let result = Tokenizer::tokenize(input);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), ErrorKind::InvalidInput);
}
