use crate::pass::PasswordEntry;
use crate::{
    crypto::{encrypt_contents, get_cipher, get_random_salt},
    pass::Passwords,
};
use aes_gcm::aead::Aead;
use std::path::Path;

pub struct PasswordStore {
    pub file_name: String,
    master_password: String,
    passwords: Option<Passwords>,
}

impl PasswordStore {
    pub fn new(file_name: &str, master_password: String) -> anyhow::Result<Self> {
        let password_json = Path::new(file_name);
        if !password_json.exists() {
            let salt = get_random_salt();
            println!("Salt generated: {:?}", salt);
            let (empty_json, nonce) = encrypt_contents("[]", &master_password, &salt);
            let mut content = salt.to_vec();
            content.extend(nonce);
            content.extend(empty_json);
            std::fs::write(file_name, content)?;
        }
        let store = Self {
            file_name: file_name.to_owned(),
            master_password,
            passwords: None,
        };
        Ok(store)
    }

    pub fn load_passwords(mut self) -> anyhow::Result<Self> {
        let encrypted_file = std::fs::read(&self.file_name)?;
        let salt = &encrypted_file[..16];
        let cipher = get_cipher(&self.master_password, salt);
        let nonce = &encrypted_file[16..28];
        let encrypted_data = &encrypted_file[28..];
        let plain_text = cipher
            .decrypt(nonce.into(), encrypted_data.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to decrypt passwords to plain text: {:?}", e))?;
        let plain_text_str = String::from_utf8(plain_text)?;
        // println!("Plain text: {}", plain_text_str);
        let parsed_passwords = Passwords::parse_passwords(&plain_text_str)?;
        self.passwords = Some(parsed_passwords);
        Ok(self)
    }

    pub fn store_passwords(self) -> anyhow::Result<Self> {
        let encrypted_file = std::fs::read(&self.file_name)?;
        let salt = &encrypted_file[..16];
        let cipher = get_cipher(&self.master_password, salt);
        let nonce = &encrypted_file[16..28];
        let plain_text = serde_json::to_string(&self.passwords)?;
        let encrypted_text = cipher
            .encrypt(nonce.into(), plain_text.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to encrypt passwords: {:?}", e))?;
        let mut content = salt.to_vec();
        content.extend(nonce);
        content.extend(encrypted_text);
        std::fs::write(&self.file_name, content)?;
        Ok(self)
    }

    pub fn add_password(
        mut self,
        service: String,
        username: Option<String>,
        password: String,
    ) -> anyhow::Result<Self> {
        let new_password = PasswordEntry::new(service, username, password);
        if let Some(ref mut passwords) = self.passwords {
            passwords.append(new_password);
        } else {
            panic!("Load passwords before appending")
        }
        Ok(self)
    }

    pub fn remove_password(mut self, service: String, username: Option<String>) -> Self {
        if let Some(_password) = self
            .passwords
            .as_mut()
            .and_then(|passwords| passwords.remove(service, username))
        {
            println!("Password deleted");
        } else {
            println!("Password not found")
        }
        self
    }

    pub fn find_password(
        &self,
        service: String,
        username: Option<String>,
    ) -> Option<&PasswordEntry> {
        self.passwords
            .as_ref()
            .and_then(|passwords| passwords.find(service, username))
    }

    pub fn list_passwords(&self) {
        if let Some(passwords) = self.passwords.as_ref() {
            passwords.print_all();
        } else {
            println!("No passwords found!")
        }
    }
}
