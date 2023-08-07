use clap::Parser;
use colored::*;
use lockbox::cli::{
    args::{Args, Command},
    commands::{
        add_password, generate_password, get_random_password, list_passwords, remove_password,
        show_password,
    },
};

fn main() {
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
            let password = if generate {
                Some(get_random_password(
                    length, symbols, uppercase, lowercase, numbers,
                ))
            } else {
                password
            };
            match add_password(file_name, service, username, master, password) {
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
    }
}
