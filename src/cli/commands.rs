use crate::{
    cli::{args::Length, io::read_input},
    store::PasswordStore,
};
use clipboard::{ClipboardContext, ClipboardProvider};
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
    let password = if let Some(password) = password {
        let ctx_result: Result<ClipboardContext, _> = ClipboardProvider::new();
        if let Ok(mut ctx) = ctx_result {
            if ctx.set_contents(password.to_owned()).is_ok() {
                println!("Random password generated and copied to clipboard")
            } else {
                println!("Random password generated");
                println!("Note: Failed to copy password to clipboard");
            }
        }
        password
    } else {
        read_input("password")
    };
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
            Ok(password) => {
                let ctx_result: Result<ClipboardContext, _> = ClipboardProvider::new();
                if let Ok(mut ctx) = ctx_result {
                    if ctx.set_contents(password.to_owned()).is_ok() {
                        println!("{} (Copied to Clipboard)", password);
                    } else {
                        println!("{}", password);
                        println!("Note: Failed to copy password to clipboard");
                    }
                } else {
                    println!("{}", password)
                }
            }
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
        password.print_password();
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
        .print_passwords(show_passwords);
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
