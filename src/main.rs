use std::{fs, str::FromStr};

use clap::Parser;
use cli::handle_cli_test;
use futures::executor;
use http::handle_http_test;
use logos::Logos;
use owo_colors::OwoColorize;
use reqwest::{Client, Method};
use runner_context::RunnerContext;
use token::Token; // 0.3.1

mod cli;
mod http;
mod runner_context;
mod token;

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
struct Args {
    file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(&args.file).expect("Could not read file");
    let mut context = RunnerContext {
        lexer: Token::lexer(&input),
        http_client: Client::new(),
        base_url: String::new(),
        indent: 0,
        indent_character: "    ",
    };
    for token in context.clone().lexer {
        match token {
            Token::BaseUrl(url) => context.base_url = url,
            Token::Test(test_name) => {
                print!("{}", context.indent_character.repeat(context.indent));
                println!("{}", test_name.bold());
                context.indent += 1;
            }
            Token::End => {
                context.indent -= 1;
            }
            Token::TestStatement((function_name, expected_value)) => {
                print!("{}", context.indent_character.repeat(context.indent));
                println!("{} returns {}", function_name, expected_value);
            }
            Token::HTTPTest((method, url, expected_value)) => {
                handle_http_test(&mut context, &method, &url, &expected_value).await;
            }
            Token::CLITest((command, expected_value)) => {
                handle_cli_test(&mut context, &command, &expected_value);
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
