use crate::{
    cli::{args::Length, io::read_input},
    store::PasswordStore,
};
use passwords::PasswordGenerator;

const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";

pub fn add_password(
    service: String,
    username: Option<String>,
    master: Option<String>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    let password = password.unwrap_or_else(|| read_input("password"));
    PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME, master)?
        .load_passwords()?
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
    if count > 1 {
        match password_generator.generate(count) {
            Ok(passwords) => {
                for password in passwords {
                    println!("{}", password)
                }
            }
            Err(err) => println!("Error generating password: {}", err),
        }
    } else {
        match password_generator.generate_one() {
            Ok(password) => println!("{}", password),
            Err(err) => println!("Error generating password: {}", err),
        }
    }
}

pub fn show_password(
    service: String,
    username: Option<String>,
    master: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    let passwords = PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME, master)?.load_passwords()?;
    let password = passwords.find_password(service, username);
    println!("Password: {:?}", password);
    Ok(())
}

pub fn list_passwords(master: Option<String>, show_passwords: bool) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME, master)?
        .load_passwords()?
        .list_passwords(show_passwords);
    Ok(())
}

pub fn remove_password(
    service: String,
    username: Option<String>,
    master: Option<String>,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    PasswordStore::new(DEFAULT_PASSWORD_FILE_NAME, master)?
        .load_passwords()?
        .remove_password(service, username)
        .store_passwords()?;
    Ok(())
}
