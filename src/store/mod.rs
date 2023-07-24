use crate::{
    crypto::{encrypt_contents, get_cipher, get_random_salt},
    pass::Passwords,
};
use aes_gcm::aead::Aead;
use std::path::Path;

pub fn initialize_password_file(
    file_name: &str,
    master_password: String,
) -> Result<(), anyhow::Error> {
    let password_json = Path::new(file_name);
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

pub fn load_passwords(
    file_name: &str,
    master_password: String,
) -> Result<Passwords, anyhow::Error> {
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

pub fn store_passwords(
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
