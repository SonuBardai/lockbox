use std::{num::NonZeroU32, path::Path};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use lockbox::cli::{build_parser, Command};
use passwords::PasswordGenerator;
use ring::rand::SystemRandom;
use ring::{pbkdf2, rand::SecureRandom};
use serde::{Deserialize, Serialize};

const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";

fn get_random_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    let r = SystemRandom::new();
    r.fill(&mut salt).unwrap();
    salt
}

fn derive_encryption_key(master_password: String, salt: &[u8]) -> [u8; 32] {
    let mut enc_key: [u8; 32] = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(100_000).unwrap(),
        salt,
        &master_password.as_bytes(),
        &mut enc_key,
    );
    enc_key
}

fn get_cipher(master_password: String, salt: &[u8]) -> Aes256Gcm {
    let enc_key = derive_encryption_key(master_password, salt);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&enc_key));
    cipher
}

fn encrypt_contents(contents: &str, master_password: String, salt: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = get_cipher(master_password, salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    println!("Nonce generated: {:?}", nonce);
    let encrypted_text = cipher.encrypt(&nonce, contents.as_ref());
    (encrypted_text.unwrap(), nonce.to_vec())
}

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

#[derive(Debug, Serialize, Deserialize)]
struct PasswordEntry {
    service: String,
    username: Option<String>,
    password: String,
}

impl PasswordEntry {
    pub fn new(service: String, username: Option<String>, password: String) -> PasswordEntry {
        PasswordEntry {
            service,
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Passwords(Vec<PasswordEntry>);

impl Passwords {
    pub fn append(&mut self, new_password: PasswordEntry) {
        self.0.push(new_password);
    }
    pub fn find(&self, service: &str, username: Option<String>) -> Option<&PasswordEntry> {
        self.0
            .iter()
            .find(|pwd| pwd.service == service && pwd.username == username)
    }
}

fn parse_passwords(raw_passwords: &str) -> Result<Passwords, anyhow::Error> {
    let passwords: Passwords = serde_json::from_str(raw_passwords)?;
    Ok(passwords)
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
    println!("Plain text: {}", plain_text_str);
    let parsed_passwords = parse_passwords(&plain_text_str)?;
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
            println!("Password: {:?}", passwords);
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
            let passwords = load_passwords(DEFAULT_PASSWORD_FILE_NAME, master.clone())
                .expect("Failed to read passwords store");
            if let Some(password) = passwords.find(&service, username.clone()) {
                println!("Password: {}", password.password);
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
