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

    pub fn list_passwords(&self, show_passwords: bool) {
        if let Some(passwords) = self.passwords.as_ref() {
            passwords.print_all(show_passwords);
        } else {
            println!("No passwords found!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const TEST_FILE_NAME: &str = "test_passwords";
    const TEST_MASTER_PASSWORD: &str = "test_master";

    #[test]
    fn test_new_password_store() {
        let store = PasswordStore::new(TEST_FILE_NAME, TEST_MASTER_PASSWORD.to_string()).unwrap();
        assert_eq!(store.file_name, TEST_FILE_NAME);
        assert_eq!(store.master_password, TEST_MASTER_PASSWORD);
        assert!(store.passwords.is_none());
        assert!(Path::new(TEST_FILE_NAME).exists());
        cleanup();
    }

    #[test]
    fn test_load_passwords() {
        let store = PasswordStore::new(TEST_FILE_NAME, TEST_MASTER_PASSWORD.to_string())
            .unwrap()
            .load_passwords()
            .unwrap();
        assert!(store.passwords.is_some());
        let expected_passwords = Passwords::new();
        assert_eq!(store.passwords.unwrap(), expected_passwords);
        cleanup();
    }

    fn cleanup() {
        fs::remove_file(TEST_FILE_NAME).unwrap();
    }
}
