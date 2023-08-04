use crate::{
    cli::{args::Length, io::read_input},
    store::PasswordStore,
};
use colored::*;
use passwords::PasswordGenerator;

pub fn add_password(
    file_name: String,
    service: String,
    username: Option<String>,
    master: Option<String>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    let password_store = PasswordStore::new(file_name, master)?.load_passwords()?;
    let password = password.unwrap_or_else(|| read_input("password"));
    password_store
        .add_password(service, username, password)?
        .store_passwords()?;
    Ok(())
}

pub fn get_random_password(
    length: Length,
    symbols: bool,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
) -> String {
    PasswordGenerator::new()
        .length(length.get_val())
        .lowercase_letters(lowercase)
        .uppercase_letters(uppercase)
        .numbers(numbers)
        .symbols(symbols)
        .strict(true)
        .generate_one()
        .unwrap()
}

pub fn generate_password(
    length: Length,
    symbols: bool,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    count: usize,
) {
    let password_generator = PasswordGenerator::new()
        .length(length.get_val())
        .lowercase_letters(lowercase)
        .uppercase_letters(uppercase)
        .numbers(numbers)
        .symbols(symbols)
        .strict(true);
    println!();
    if count > 1 {
        match password_generator.generate(count) {
            Ok(passwords) => {
                for password in passwords {
                    println!("{}", password.green())
                }
            }
            Err(err) => println!("Error generating password: {}", err),
        }
    } else {
        match password_generator.generate_one() {
            Ok(password) => println!("{}", password.green()),
            Err(err) => println!("Error generating password: {}", err),
        }
    }
}

pub fn show_password(
    file_name: String,
    service: String,
    username: Option<String>,
    master: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    let passwords = PasswordStore::new(file_name, master)?.load_passwords()?;
    let password = passwords.find_password(service, username);
    if let Some(password) = password {
        password.print_password(Some(Color::Blue));
    } else {
        println!("Password not found");
    }
    Ok(())
}

pub fn list_passwords(
    file_name: String,
    master: Option<String>,
    show_passwords: bool,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    PasswordStore::new(file_name, master)?
        .load_passwords()?
        .print_passwords(show_passwords, Some(Color::Blue));
    Ok(())
}

pub fn remove_password(
    file_name: String,
    service: String,
    username: Option<String>,
    master: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    PasswordStore::new(file_name, master)?
        .load_passwords()?
        .remove_password(service, username)
        .store_passwords()?;
    Ok(())
}
