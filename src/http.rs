use std::str::FromStr;

use owo_colors::OwoColorize;
use reqwest::Method;

use crate::runner_context::RunnerContext;

pub async fn handle_http_test(
    context: &mut RunnerContext<'_>,
    method: &str,
    url: &str,
    expected_value: &str,
) {
    print!("{}", context.indent_character.repeat(context.indent));
    println!("{} {}", method, context.base_url.to_owned() + &url,);
    let resp = context
        .http_client
        .request(
            Method::from_str(method).expect("Invalid HTTP Method"),
            context.base_url.to_owned() + &url,
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
        print!("{}", context.indent_character.repeat(context.indent + 1));
        println!("{}", fail_reason.red());
    } else {
        print!("{}", context.indent_character.repeat(context.indent + 1));
        println!("{}", "Passed".green());
    }
}
