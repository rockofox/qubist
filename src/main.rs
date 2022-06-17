use std::fs;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    #[regex(r"[\t\n\f]+", logos::skip)]
    Error,

    #[regex(r"Test \w+", test)]
    Test,

    #[regex(r"[\w()]+ returns .+", test_statement)]
    TestStatement,

    #[regex(r"(GET|POST|DELETE) .+", http_test)]
    HTTPTest,
}
fn test(lex: &mut logos::Lexer<Token>) {
    println!("{}", lex.slice().split(" ").last().unwrap());
}
fn test_statement(lex: &mut logos::Lexer<Token>) {
    let statement = lex.slice();
    let function_name = statement.split(" ").nth(0).unwrap();
    let expected_value = statement.split(" ").nth(2).unwrap();
    println!("assert({}, {})", function_name, expected_value);
}
fn http_test(lex: &mut logos::Lexer<Token>) {
    let statement = lex.slice();
    let method = statement.split(" ").nth(0).unwrap();
    let url = statement.split(" ").nth(1).unwrap();
    let expected_value = statement.split(" ").nth(3).unwrap();
    println!("{} {} -> {}", method, url, expected_value);
}
fn main() {
    let input = fs::read_to_string("sample.tst").expect("Could not read file");
    let mut lex = Token::lexer(&input);

    while lex.next() != None {}
}
