use lockbox::{
    cli::{build_parser, Command, Length},
    pass::PasswordEntry,
    store::{initialize_password_file, load_passwords, store_passwords},
};
use passwords::PasswordGenerator;

const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";

fn add_password(service: String, username: Option<String>, master: String, password: String) {
    initialize_password_file(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to initialize passwords store");
    let mut passwords = load_passwords(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to read passwords store");
    // println!("Password: {:?}", passwords);
    let new_password = PasswordEntry::new(service, username, password);
    passwords.append(new_password);
    store_passwords(DEFAULT_PASSWORD_FILE_NAME, master, passwords)
        .expect("Failed to store new password");
}

fn generate_password(
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

fn show_password(service: String, username: Option<String>, master: String) {
    initialize_password_file(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to initialize passwords store");
    let passwords =
        load_passwords(DEFAULT_PASSWORD_FILE_NAME, master).expect("Failed to read passwords store");
    if let Some(password) = passwords.find(&service, username.clone()) {
        password.print_password();
    } else {
        print!("Cannot find the given service {}", service);
        if let Some(u) = username {
            print!(" and username {}", u);
        }
        println!()
    }
}

fn list_passwords(master: String) {
    initialize_password_file(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to initialize passwords store");
    let passwords =
        load_passwords(DEFAULT_PASSWORD_FILE_NAME, master).expect("Failed to read passwords store");
    passwords.print_all();
}

fn remove_password(service: String, username: Option<String>, master: String) {
    initialize_password_file(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to initialize passwords store");
    let mut passwords = load_passwords(DEFAULT_PASSWORD_FILE_NAME, master.clone())
        .expect("Failed to read passwords store");
    if passwords.remove(&service, username.clone()) {
        store_passwords(DEFAULT_PASSWORD_FILE_NAME, master, passwords)
            .expect("Failed to store new password");
        println!("Password deleted");
    } else {
        print!("Cannot find the given service {}", service);
        if let Some(u) = username {
            print!(" and username {}", u);
        }
        println!()
    }
}

fn main() {
    let cli_args = build_parser();
    match cli_args.command {
        Command::Add {
            service,
            username,
            master,
            password,
        } => add_password(service, username, master, password),
        Command::Generate {
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
        } => generate_password(length, symbols, uppercase, lowercase, numbers, count),
        Command::List { master } => list_passwords(master),
        Command::Remove {
            service,
            username,
            master,
        } => remove_password(service, username, master),
        Command::Show {
            service,
            username,
            master,
        } => show_password(service, username, master),
    }
}
