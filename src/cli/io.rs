use colored::*;
use rpassword::prompt_password;
use std::io::{self, Write};

pub fn read_hidden_input(prompt: &str) -> String {
    let input = prompt_password(format!("Please enter the {}\n{}", prompt, ">> ".yellow()))
        .unwrap_or_else(|_| panic!("Failed to read {}", prompt));
    input.trim().to_string()
}

pub fn read_terminal_input(prompt: Option<&str>) -> String {
    if let Some(prompt) = prompt {
        println!("{prompt}");
    }
    print!("{}", ">> ".yellow());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}
