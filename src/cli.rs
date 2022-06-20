use owo_colors::OwoColorize;

use crate::runner_context::RunnerContext;

pub fn handle_cli_test(context: &mut RunnerContext<'_>, command: &str, expected_value: &str) {
    let mut sanitized_command = command
        .split('\n')
        .map(|s| {
            return context.indent_character.repeat(context.indent) + s.trim();
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
        print!("{}", context.indent_character.repeat(context.indent + 1));
        println!("{}", fail_reason.red());
    } else {
        print!("{}", context.indent_character.repeat(context.indent + 1));
        println!("{}", "Passed".green());
    }
}
