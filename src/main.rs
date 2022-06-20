use std::{fs, str::FromStr};

use clap::Parser;
use logos::Logos;
use owo_colors::OwoColorize;
use reqwest::{Client, Method}; // 0.3.1

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"Test [\w ]+", test)]
    Test(String),

    #[regex(r"End")]
    End,

    #[regex(r"Function [\w()]+ returns .+", test_statement)]
    TestStatement((String, String)),

    #[regex(r"Executing `[^`]+` returns .+", cli_test)]
    CLITest((String, String)),

    #[regex(r"Base URL is .+", base_url)]
    BaseUrl(String),

    #[regex(
        r"(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) .+",
        http_test
    )]
    HTTPTest((String, String, String)),
    #[regex(r"#")]
    Comment,

    #[error]
    // #[regex(r"", logos::skip)]
    Error,
}

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
struct Args {
    file: String,
}

fn cli_test(lex: &mut logos::Lexer<Token>) -> Option<(String, String)> {
    let statement = lex.slice();
    let command = statement
        .chars()
        .skip_while(|c| *c != '`')
        .skip(1)
        .take_while(|c| *c != '`')
        .collect::<String>();
    let expected_value = statement.split("returns ").nth(1).unwrap();

    Some((command, expected_value.to_string()))
}

fn base_url(lex: &mut logos::Lexer<Token>) -> Option<String> {
    lex.slice()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .map(|s| s.to_string())
}
fn test(lex: &mut logos::Lexer<Token>) -> Option<String> {
    Some(lex.slice().split(' ').last().unwrap().to_string())
}
fn test_statement(lex: &mut logos::Lexer<Token>) -> Option<(String, String)> {
    let statement = lex.slice();
    let function_name = statement.split(' ').nth(1).unwrap();
    let expected_value = statement
        .split(' ')
        .skip(3)
        .collect::<Vec<&str>>()
        .join(" ");
    Some((function_name.to_string(), expected_value))
}
fn http_test(lex: &mut logos::Lexer<Token>) -> Option<(String, String, String)> {
    let statement = lex.slice();
    let method = statement.split(' ').next().unwrap();
    let url = statement.split(' ').nth(1).unwrap();
    let expected_value = statement
        .split(' ')
        .skip(3)
        .collect::<Vec<&str>>()
        .join(" ");
    Some((method.to_string(), url.to_string(), expected_value))
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(&args.file).expect("Could not read file");
    let lex = Token::lexer(&input);
    let mut base_url = String::new();
    let http_client = Client::new();
    let mut indent = 0;
    let indent_character = "    ";
    for token in lex {
        match token {
            Token::BaseUrl(url) => base_url = url,
            Token::Test(test_name) => {
                print!("{}", indent_character.repeat(indent));
                println!("{}", test_name.bold());
                indent += 1;
            }
            Token::End => {
                indent -= 1;
            }
            Token::TestStatement((function_name, expected_value)) => {
                print!("{}", indent_character.repeat(indent));
                println!("{} returns {}", function_name, expected_value);
            }
            Token::HTTPTest((method, url, expected_value)) => {
                print!("{}", indent_character.repeat(indent));
                println!("{} {}", method, base_url.to_owned() + &url,);
                let resp = http_client
                    .request(
                        Method::from_str(method.as_str()).expect("Invalid HTTP Method"),
                        base_url.to_owned() + &url,
                    )
                    .header("Accept", "application/json")
                    .send()
                    .await;
                let mut fail_reason = String::new();

                match resp {
                    Ok(x) => {
                        let body = x.text().await.unwrap();
                        if body != expected_value {
                            fail_reason.push_str("Response did not match expected value");
                            fail_reason.push('\n');
                            fail_reason.push_str(&format!("Expected: {}", expected_value));
                            fail_reason.push('\n');
                            fail_reason.push_str(&format!("Actual: {}", body));
                        }
                    }
                    Err(e) => {
                        fail_reason.push_str(&format!("{}", e));
                    }
                }
                if !fail_reason.is_empty() {
                    print!("{}", indent_character.repeat(indent + 1));
                    println!("{}", fail_reason.red());
                } else {
                    print!("{}", indent_character.repeat(indent + 1));
                    println!("{}", "Passed".green());
                }
            }
            Token::CLITest((command, expected_value)) => {
                let mut sanitized_command = command
                    .split('\n')
                    .map(|s| {
                        return indent_character.repeat(indent) + s.trim();
                    })
                    .collect::<Vec<_>>();
                sanitized_command.retain(|s| !s.trim().is_empty());
                let sanitized_command = sanitized_command.join("\n");
                println!("{}", sanitized_command);
                let command = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output();
                let mut fail_reason = String::new();
                match command {
                    Ok(x) => {
                        if std::str::from_utf8(&x.stdout).unwrap() != expected_value {
                            fail_reason.push_str("Response body did not match expected value");
                            fail_reason.push('\n');
                            fail_reason.push_str(&format!("Expected: {}", expected_value));
                            fail_reason.push('\n');
                            fail_reason.push_str(&format!(
                                "Actual: {}",
                                std::str::from_utf8(&x.stdout).unwrap()
                            ));
                        }
                    }
                    Err(e) => {
                        fail_reason.push_str(&format!("{}", e));
                    }
                }
                if !fail_reason.is_empty() {
                    print!("{}", indent_character.repeat(indent + 1));
                    println!("{}", fail_reason.red());
                } else {
                    print!("{}", indent_character.repeat(indent + 1));
                    println!("{}", "Passed".green());
                }
            }
            Token::Comment => {}
            Token::Error => {
                // println!("??? {:?}", token);
            }
            _ => {
                println!("{:?}", token);
            }
        }
    }
}
