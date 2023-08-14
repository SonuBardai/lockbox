use clap::Parser;
use colored::*;
use lockbox::{
    cli::{
        args::{Args, Command, DEFAULT_PASSWORD_FILE_NAME},
        commands::{
            add_password, generate_password, list_passwords, remove_password, show_password,
        },
        io::{read_hidden_input, RpasswordPromptPassword},
    },
    repl::repl,
    store::PasswordStore,
};
use passwords::PasswordGenerator;

fn main() {
    if std::env::args().len() == 1 {
        repl(
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
            &mut std::io::stdout(),
            &RpasswordPromptPassword,
        )
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
                let master = master.unwrap_or_else(|| {
                    read_hidden_input("master password", &RpasswordPromptPassword)
                });
                let mut password_store = match PasswordStore::new(file_name, master) {
                    Ok(password_store) => password_store,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                };
                match add_password(
                    &mut password_store,
                    service,
                    username,
                    password,
                    generate,
                    password_generator,
                ) {
                    Ok(_) => println!("{}", "Password added successfully".green()),
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
            } => match generate_password(
                length,
                symbols,
                uppercase,
                lowercase,
                numbers,
                count,
                &mut std::io::stdout(),
            ) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
            },
            Command::List {
                file_name,
                master,
                show_passwords,
            } => {
                let master = master.unwrap_or_else(|| {
                    read_hidden_input("master password", &RpasswordPromptPassword)
                });
                let mut password_store = match PasswordStore::new(file_name, master) {
                    Ok(password_store) => password_store,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                };
                match list_passwords(&mut password_store, show_passwords, &mut std::io::stdout()) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                }
            }
            Command::Remove {
                file_name,
                service,
                username,
                master,
            } => {
                let master = master.unwrap_or_else(|| {
                    read_hidden_input("master password", &RpasswordPromptPassword)
                });
                let mut password_store = match PasswordStore::new(file_name, master) {
                    Ok(password_store) => password_store,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                };
                match remove_password(
                    &mut std::io::stdout().lock(),
                    &mut password_store,
                    service,
                    username,
                ) {
                    Ok(_) => println!("Password removed successfully"),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                }
            }
            Command::Show {
                file_name,
                service,
                username,
                master,
            } => {
                let master = master.unwrap_or_else(|| {
                    read_hidden_input("master password", &RpasswordPromptPassword)
                });
                let mut password_store = match PasswordStore::new(file_name, master) {
                    Ok(password_store) => password_store,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                };
                match show_password(
                    &mut password_store,
                    service,
                    username,
                    &mut std::io::stdout(),
                ) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", format!("Error: {}", err).red()),
                }
            }
            Command::Repl => repl(
                DEFAULT_PASSWORD_FILE_NAME.to_string(),
                &mut std::io::stdout(),
                &RpasswordPromptPassword,
            ),
        }
    }
}
