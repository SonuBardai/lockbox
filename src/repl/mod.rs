use crate::{
    cli::{
        args::{Length, DEFAULT_PASSWORD_FILE_NAME},
        commands::copy_to_clipboard,
        io::{read_hidden_input, read_terminal_input},
    },
    store::PasswordStore,
};
use colored::*;
use passwords::PasswordGenerator;

pub fn repl() {
    println!("Welcome to L🦀CKBOX!\n");
    let master = read_hidden_input("master password");
    let mut password_store = PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME.to_string(), master)
        .unwrap_or_else(|_| panic!("{}", "Failed to initialize password store".red()));
    loop {
        println!("\nEnter [1] add password [2] generate random password [3] list passwords [4] remove password [5] show password [6] exit");
        let input = read_terminal_input(None);
        match input.as_str() {
            "1" | "add" | "a" => {
                println!(
                    "Enter [1] randomly generate password [2] add your own password [3] cancel"
                );
                let input = read_terminal_input(None);
                let password = match input.as_str() {
                    "1" | "generate" | "g" | "random" | "r" => {
                        let password = PasswordGenerator::new()
                            .length(Length::Sixteen.get_val())
                            .lowercase_letters(true)
                            .uppercase_letters(true)
                            .numbers(true)
                            .symbols(false)
                            .strict(true)
                            .generate_one()
                            .unwrap_or_else(|_| panic!("{}", "Failed to generate password".red()));
                        if copy_to_clipboard(password.clone()).is_ok() {
                            println!("{} (Copied to clipboard)", password.green())
                        } else {
                            println!("{}", password.green())
                        }
                        password
                    }
                    "2" | "enter" | "e" => read_hidden_input("password"),
                    _ => continue,
                };
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                password_store
                    .load()
                    .unwrap_or_else(|_| panic!("{}", "Failed to load passwords to store".red()))
                    .push(service, username, password)
                    .unwrap_or_else(|_| panic!("{}", "Failed to new password to store".red()))
                    .dump()
                    .unwrap_or_else(|_| {
                        panic!("{}", "Failed to dump updated passwords to store".red())
                    });
                println!("{}", "Password added successfully".green());
            }
            "2" | "generate" | "g" => {
                let password = PasswordGenerator::new()
                    .length(Length::Sixteen.get_val())
                    .lowercase_letters(true)
                    .uppercase_letters(true)
                    .numbers(true)
                    .symbols(false)
                    .strict(true)
                    .generate_one()
                    .unwrap_or_else(|_| panic!("{}", "Failed to generate password".red()));
                if copy_to_clipboard(password.clone()).is_ok() {
                    println!("{} (Copied to clipboard)", password.green())
                } else {
                    println!("{}", password.green())
                }
            }
            "3" | "list" | "l" => {
                password_store
                    .load()
                    .unwrap_or_else(|_| panic!("{}", "Failed to load passwords to store".red()))
                    .print(true, Some(Color::Blue));
            }
            "4" | "remove" | "r" => {
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                password_store
                    .load()
                    .unwrap_or_else(|_| panic!("{}", "Failed to load passwords to store".red()))
                    .pop(service, username)
                    .dump()
                    .unwrap_or_else(|_| {
                        panic!("{}", "Failed to dump updated passwords to store".red())
                    });
            }
            "5" | "show" | "s" => {
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                let password = password_store
                    .load()
                    .unwrap_or_else(|_| panic!("{}", "Failed to load passwords to store".red()))
                    .find(service, username);
                if let Some(password) = password {
                    password.print_password(Some(Color::Blue));
                } else {
                    println!("Password not found");
                }
            }
            "6" | "exit" | "e" | _ => break,
        }
    }
}
