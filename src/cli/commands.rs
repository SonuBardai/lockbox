use crate::{
    cli::{args::Length, io::read_input},
    store::PasswordStore,
};
use anyhow::anyhow;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use passwords::PasswordGenerator;

fn copy_to_clipboard(password: String) -> anyhow::Result<()> {
    let ctx_result: Result<ClipboardContext, _> = ClipboardProvider::new();
    let mut ctx = ctx_result.map_err(|_| anyhow!("Unable to initialize clipboard"))?;
    ctx.set_contents(password)
        .map_err(|_| anyhow!("Unable to set clipboard contents"))?;
    Ok(())
}

pub fn add_password(
    file_name: String,
    service: String,
    username: Option<String>,
    master: Option<String>,
    password: Option<String>,
    generate: bool,
    password_generator: PasswordGenerator,
) -> anyhow::Result<()> {
    let master = master.unwrap_or_else(|| read_input("master password"));
    let password_store = PasswordStore::new(file_name, master)?.load()?;
    let password = if generate {
        let password = password_generator
            .generate_one()
            .expect(&format!("{}", "Failed to generate password".red()));
        if copy_to_clipboard(password.clone()).is_ok() {
            println!("Random password generated and copied to clipboard");
        } else {
            println!("Random password generated");
            println!("Note: Failed to copy password to clipboard");
        }
        password
    } else {
        password.unwrap_or_else(|| read_input("password"))
    };
    password_store.push(service, username, password)?.dump()?;
    Ok(())
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
            Ok(password) => {
                if copy_to_clipboard(password.clone()).is_ok() {
                    println!("{} (Copied to Clipboard)", password.green());
                } else {
                    println!("{}", password.green());
                    println!("Note: Failed to copy password to clipboard");
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
    let passwords = PasswordStore::new(file_name, master)?.load()?;
    let password = passwords.find(service, username);
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
        .load()?
        .print(show_passwords, Some(Color::Blue));
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
        .load()?
        .pop(service, username)
        .dump()?;
    Ok(())
}
