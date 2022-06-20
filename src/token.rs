use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
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
