use std::{fs, str::FromStr};

use crate::token::Token;
use clap::Parser;
use futures::executor;
use logos::Logos;
use owo_colors::OwoColorize;
use reqwest::{Client, Method}; // 0.3.1
#[derive(Clone)]
pub struct RunnerContext<'a> {
    pub lexer: logos::Lexer<'a, Token>,
    pub http_client: Client,
    pub base_url: String,
    pub indent: usize,
    pub indent_character: &'a str,
}
