use clap::Parser;
use lockbox::cli::{
    args::{Args, Command},
    commands::{
        add_password, generate_password, get_random_password, list_passwords, remove_password,
        show_password,
    },
};

const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Add {
            service,
            username,
            password,
            master,
            generate,
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
        } => {
            let password = if generate {
                Some(get_random_password(
                    length, symbols, uppercase, lowercase, numbers,
                ))
            } else {
                password
            };
            add_password(
                DEFAULT_PASSWORD_FILE_NAME.to_string(),
                service,
                username,
                master,
                password,
            )
        }
        .expect("Failed to add password"),
        Command::Generate {
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
        } => generate_password(length, symbols, uppercase, lowercase, numbers, count),
        Command::List {
            master,
            show_passwords,
        } => list_passwords(
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
            master,
            show_passwords,
        )
        .expect("Failed to get passwords"),
        Command::Remove {
            service,
            username,
            master,
        } => remove_password(
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
            service,
            username,
            master,
        )
        .expect("Failed to remove password"),
        Command::Show {
            service,
            username,
            master,
        } => show_password(
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
            service,
            username,
            master,
        )
        .expect("Failed to get passwords"),
    }
}
