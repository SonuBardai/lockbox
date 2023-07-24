use aes_gcm::aead::Aead;
use lockbox::{
    cli::{build_parser, Command},
    crypto::{encrypt_contents, get_cipher, get_random_salt},
    pass::{PasswordEntry, Passwords},
};
use passwords::PasswordGenerator;
use std::path::Path;

const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";

fn initialize_password_file(file_name: &str, master_password: String) -> Result<(), anyhow::Error> {
    let password_json = Path::new(DEFAULT_PASSWORD_FILE_NAME);
    if !password_json.exists() {
        let salt = get_random_salt();
        println!("Salt generated: {:?}", salt);
        let (empty_json, nonce) = encrypt_contents("[]", master_password, &salt);
        let mut content = salt.to_vec();
        content.extend(nonce);
        content.extend(empty_json);
        std::fs::write(file_name, content)?;
    }
    Ok(())
}

fn load_passwords(file_name: &str, master_password: String) -> Result<Passwords, anyhow::Error> {
    let encrypted_file = std::fs::read(file_name)?;
    let salt = &encrypted_file[..16];
    let cipher = get_cipher(master_password, salt);
    let nonce = &encrypted_file[16..28];
    let encrypted_data = &encrypted_file[28..];
    let plain_text = cipher
        .decrypt(nonce.into(), encrypted_data.as_ref())
        .expect("Failed to decrypt data");
    let plain_text_str = String::from_utf8(plain_text)?;
    // println!("Plain text: {}", plain_text_str);
    let parsed_passwords = Passwords::parse_passwords(&plain_text_str)?;
    Ok(parsed_passwords)
}

fn store_passwords(
    file_name: &str,
    master_password: String,
    passwords: Passwords,
) -> Result<(), anyhow::Error> {
    let encrypted_file = std::fs::read(file_name)?;
    let salt = &encrypted_file[..16];
    let cipher = get_cipher(master_password, salt);
    let nonce = &encrypted_file[16..28];
    let plain_text = serde_json::to_string(&passwords)?;
    let encrypted_text = cipher
        .encrypt(nonce.into(), plain_text.as_ref())
        .expect("Failed to encrypt in store_passwords");
    let mut content = salt.to_vec();
    content.extend(nonce);
    content.extend(encrypted_text);
    std::fs::write(file_name, content)?;
    Ok(())
}

fn main() {
    let args = build_parser();
    match args.command {
        Command::Add {
            service,
            username,
            master,
            password,
        } => {
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
        Command::Generate {
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
        } => {
            let pg = PasswordGenerator::new()
                .length(length.get_val())
                .lowercase_letters(lowercase)
                .uppercase_letters(uppercase)
                .numbers(numbers)
                .symbols(symbols)
                .strict(true);
            if count > 1 {
                match pg.generate(count) {
                    Ok(passwords) => {
                        for password in passwords {
                            println!("{}", password)
                        }
                    }
                    Err(err) => println!("Error generating password: {}", err),
                }
            } else {
                match pg.generate_one() {
                    Ok(password) => println!("{}", password),
                    Err(err) => println!("Error generating password: {}", err),
                }
            }
        }
        Command::List => {
            println!("List operation.");
        }
        Command::Remove { service, username } => {
            println!("Add operation.");
            println!("Service: {:?}, Username: {}", service, username);
        }
        Command::Show {
            service,
            username,
            master,
        } => {
            initialize_password_file(DEFAULT_PASSWORD_FILE_NAME, master.clone())
                .expect("Failed to initialize passwords store");
            let passwords = load_passwords(DEFAULT_PASSWORD_FILE_NAME, master)
                .expect("Failed to read passwords store");
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
    }
}
