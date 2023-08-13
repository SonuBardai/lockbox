use crate::{
    cli::{
        args::{Length, DEFAULT_PASSWORD_FILE_NAME},
        commands::{
            add_password, generate_password, list_passwords, remove_password, show_password,
        },
        io::{read_hidden_input, read_terminal_input},
    },
    store::PasswordStore,
};
use colored::*;
use passwords::PasswordGenerator;

pub fn repl() {
    println!("{}", "Welcome to L🦀CKBOX!\n".bold());
    let master = read_hidden_input("master password");
    let password_store = match PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME.to_string(), master) {
        Ok(password_store) => password_store,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    run_repl(password_store);
}

pub fn run_repl(mut password_store: PasswordStore) {
    while let Err(err) = password_store.load() {
        eprintln!("{}: {err}", "Failed to load password store".red());
        let master = read_hidden_input("master password");
        password_store.update_master(master);
    }
    loop {
        let message = [
            format!("[{}] {} password", "1".green().bold(), "add".green().bold()),
            format!(
                "[{}] {} random password",
                "2".green().bold(),
                "generate".green().bold()
            ),
            format!(
                "[{}] {} passwords",
                "3".green().bold(),
                "list".green().bold()
            ),
            format!(
                "[{}] {} password",
                "4".green().bold(),
                "remove".green().bold()
            ),
            format!(
                "[{}] {} password",
                "5".green().bold(),
                "show".green().bold()
            ),
            format!("[{}] {}", "6".green().bold(), "exit".green().bold()),
        ];
        let message = message.join(" ");
        println!("\nEnter {message}");
        let input = read_terminal_input(None);
        match input.as_str() {
            "1" | "add" | "a" => {
                let message = [
                    format!(
                        "[{}] {} random password",
                        "1".green().bold(),
                        "generate".green().bold()
                    ),
                    format!(
                        "[{}] {} your own password",
                        "2".green().bold(),
                        "enter".green().bold()
                    ),
                    format!("[{}] {}", "3".green().bold(), "cancel".green().bold()),
                ];
                let message = message.join(" ");
                println!("{}", message);
                let input = read_terminal_input(None);
                let generate = match input.as_str() {
                    "1" | "generate" | "g" | "random" | "r" => true,
                    "2" | "enter" | "e" => false,
                    _ => continue,
                };
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                let password_generator = PasswordGenerator::new()
                    .length(Length::Sixteen.get_val())
                    .lowercase_letters(true)
                    .uppercase_letters(true)
                    .numbers(true)
                    .symbols(false)
                    .strict(true);
                match add_password(
                    &mut password_store,
                    service,
                    username,
                    None,
                    generate,
                    password_generator,
                ) {
                    Ok(_) => println!("{}", "Password added successfully".green()),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                };
            }
            "2" | "generate" | "g" => {
                match generate_password(
                    Length::Sixteen,
                    false,
                    true,
                    true,
                    true,
                    1,
                    &mut std::io::stdout(),
                ) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                };
            }
            "3" | "list" | "l" => {
                list_passwords(&mut password_store, true, &mut std::io::stdout()).unwrap_or_else(
                    |err| panic!("{}: {err}", "Failed to load passwords to store".red()),
                );
            }
            "4" | "remove" | "r" => {
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                remove_password(&mut password_store, service, username).unwrap_or_else(|err| {
                    panic!(
                        "{}: {err}",
                        "Failed to dump updated passwords to store".red()
                    )
                });
            }
            "5" | "show" | "s" => {
                let service = read_terminal_input(Some("Please enter the service name"));
                let username = read_terminal_input(Some("Please enter the username (Optional)"));
                let username = Option::from(username).filter(|s| !s.is_empty());
                if show_password(
                    &mut password_store,
                    service,
                    username,
                    &mut std::io::stdout(),
                )
                .is_err()
                {
                    eprintln!("Password not found");
                };
            }
            _ => break,
        }
    }
}
