use std::{fs, str::FromStr};

use futures::executor;
use logos::{Lexer, Logos};
use owo_colors::OwoColorize;
use reqwest::{Client, Method}; // 0.3.1

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    #[regex(r"[\t\n\f]+", logos::skip)]
    Error,

    #[regex(r"Test \w+", test)]
    Test(String),

    #[regex(r"[\w()]+ returns .+", test_statement)]
    TestStatement((String, String)),
    #[regex(r"Base URL is .+", base_url)]
    BaseUrl(String),

    #[regex(
        r"(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) .+",
        http_test
    )]
    HTTPTest((String, String, String)),
}
fn base_url(lex: &mut logos::Lexer<Token>) -> Option<String> {
    lex.slice()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .map(|s| s.to_string())
}
fn test(lex: &mut logos::Lexer<Token>) -> Option<String> {
    Some(lex.slice().split(" ").last().unwrap().to_string())
}
fn test_statement(lex: &mut logos::Lexer<Token>) -> Option<(String, String)> {
    let statement = lex.slice();
    let function_name = statement.split(" ").nth(0).unwrap();
    let expected_value = statement.split(" ").nth(2).unwrap();
    Some((function_name.to_string(), expected_value.to_string()))
}
fn http_test(lex: &mut logos::Lexer<Token>) -> Option<(String, String, String)> {
    let statement = lex.slice();
    let method = statement.split(" ").nth(0).unwrap();
    let url = statement.split(" ").nth(1).unwrap();
    let expected_value = statement.split(" ").nth(3).unwrap();
    Some((
        method.to_string(),
        url.to_string(),
        expected_value.to_string(),
    ))
}
#[tokio::main]
async fn main() {
    let input = fs::read_to_string("sample.tst").expect("Could not read file");
    let mut lex = Token::lexer(&input);
    let mut base_url = String::new();
    let http_client = Client::new();
    for token in lex {
        match token {
            Token::BaseUrl(url) => base_url = url,
            Token::Test(test_name) => {
                println!("{}", test_name.bold());
            }
            Token::TestStatement((function_name, expected_value)) => {
                println!("{} returns {}", function_name, expected_value);
            }
            Token::HTTPTest((method, url, expected_value)) => {
                println!("{} {}", method, base_url.to_owned() + &url,);
                let mut resp = http_client
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
                            fail_reason.push_str("Response body did not match expected value");
                            fail_reason.push_str("\n");
                            fail_reason.push_str(&format!("Expected: {}", expected_value));
                            fail_reason.push_str("\n");
                            fail_reason.push_str(&format!("Actual: {}", body));
                        }
                    }
                    Err(e) => {
                        fail_reason.push_str(&format!("{}", e));
                    }
                }
                if fail_reason.len() > 0 {
                    println!("{}", fail_reason.red());
                } else {
                    println!("{}", "Passed".green());
                }
            }
            Token::Error => {
                // println!(".");
            }
            _ => {
                println!("{:?}", token);
            }
        }
    }
}
