use clap::Parser;
use colored::*;
use lockbox::{
    cli::{
        args::{Args, Command},
        commands::{
            add_password, generate_password, list_passwords, remove_password, show_password,
        },
    },
    repl::repl,
};
use passwords::PasswordGenerator;

fn main() {
    if std::env::args().len() == 1 {
        repl()
    } else {
        let args = Args::parse();
        match args.command {
            Command::Add {
                file_name,
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
                let password_generator = PasswordGenerator::new()
                    .length(length.get_val())
                    .lowercase_letters(lowercase)
                    .uppercase_letters(uppercase)
                    .numbers(numbers)
                    .symbols(symbols)
                    .strict(true);
                match add_password(
                    file_name,
                    service,
                    username,
                    master,
                    password,
                    generate,
                    password_generator,
                ) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                }
            }
            Command::Generate {
                length,
                symbols,
                uppercase,
                lowercase,
                numbers,
                count,
            } => generate_password(length, symbols, uppercase, lowercase, numbers, count),
            Command::List {
                file_name,
                master,
                show_passwords,
            } => match list_passwords(file_name, master, show_passwords) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
            },
            Command::Remove {
                file_name,
                service,
                username,
                master,
            } => match remove_password(file_name, service, username, master) {
                Ok(_) => println!("Password removed successfully"),
                Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
            },
            Command::Show {
                file_name,
                service,
                username,
                master,
            } => match show_password(file_name, service, username, master) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
            },
            Command::Repl => repl(),
        }
    }
}
