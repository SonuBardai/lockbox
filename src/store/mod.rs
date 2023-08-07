use crate::pass::PasswordEntry;
use crate::{
    crypto::{encrypt_contents, get_cipher, get_random_salt},
    pass::Passwords,
};
use aes_gcm::aead::Aead;
use std::fs;
use std::path::Path;

pub struct PasswordStore {
    pub file_name: String,
    master_password: String,
    passwords: Option<Passwords>,
}

impl PasswordStore {
    pub fn new(file_name: String, master_password: String) -> anyhow::Result<Self> {
        let file_path = Path::new(&file_name);
        if !file_path.exists() {
            fs::File::create(file_path)?;
        }
        if fs::metadata(&file_name)?.len() == 0 {
            let salt = get_random_salt();
            let (empty_json, nonce) = encrypt_contents("[]", &master_password, &salt);
            let mut content = salt.to_vec();
            content.extend(nonce);
            content.extend(empty_json);
            fs::write(&file_name, content)?;
        }
        let store = Self {
            file_name: file_name.to_string(),
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
            .map_err(|_| anyhow::anyhow!("Master password incorrect. Please try again."))?;
        let plain_text_str = String::from_utf8(plain_text)?;
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
        let new_password = PasswordEntry::new(service.clone(), username.clone(), password);
        if let Some(ref mut passwords) = self.passwords {
            passwords.append(new_password);
        } else {
            panic!("Load passwords before appending");
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
            println!("Password not found");
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

    pub fn print_passwords(&self, show_passwords: bool) {
        if let Some(passwords) = self.passwords.as_ref() {
            passwords.print_all(show_passwords);
        } else {
            println!("No passwords found!");
        }
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::PasswordEntry;
    use super::*;
    use rstest::rstest;
    use tempfile::NamedTempFile;

    const TEST_MASTER_PASSWORD: &str = "test_master";

    #[test]
    fn test_new_password_store() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let store =
            PasswordStore::new(temp_file_name.to_string(), TEST_MASTER_PASSWORD.to_string())
                .unwrap();
        assert_eq!(store.file_name, temp_file_name);
        assert_eq!(store.master_password, TEST_MASTER_PASSWORD);
        assert!(store.passwords.is_none());
        assert!(Path::new(temp_file_name).exists());
    }

    #[test]
    fn test_new_password_store_with_nonexistent_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file_name = temp_dir.path().join("nonexistent_file");
        let store = PasswordStore::new(
            temp_file_name.to_str().unwrap().to_string(),
            TEST_MASTER_PASSWORD.to_string(),
        )
        .unwrap();
        assert_eq!(store.file_name, temp_file_name.to_str().unwrap());
        assert_eq!(store.master_password, TEST_MASTER_PASSWORD);
        assert!(store.passwords.is_none());
        assert!(Path::new(temp_file_name.to_str().unwrap()).exists());
    }

    #[rstest]
    #[case(Vec::new())]
    #[case(
        Vec::from([
            PasswordEntry::new(
                "test_service_1".to_string(),
                Some("test_username_1".to_string()),
                "test_password_1".to_string(),
            ),
        ])
    )]
    #[case(
        Vec::from([
            PasswordEntry::new(
                "test_service_1".to_string(),
                Some("test_username_1".to_string()),
                "test_password_1".to_string(),
            ),
            PasswordEntry::new(
                "test_service_2".to_string(),
                Some("test_username_2".to_string()),
                "test_password_2".to_string(),
            ),
        ])
    )]
    fn test_load_after_store_passwords(#[case] test_passwords: Vec<PasswordEntry>) {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut store =
            PasswordStore::new(temp_file_name.to_string(), TEST_MASTER_PASSWORD.to_string())
                .unwrap()
                .load_passwords()
                .unwrap();
        test_passwords.iter().for_each(|test_password| {
            store
                .passwords
                .as_mut()
                .unwrap()
                .append(test_password.clone());
        });
        test_passwords.iter().for_each(|test_password| {
            assert_eq!(
                store
                    .passwords
                    .as_ref()
                    .unwrap()
                    .find(
                        test_password.service.clone(),
                        test_password.username.clone()
                    )
                    .unwrap(),
                test_password
            );
        });
    }

    #[rstest(
        test_passwords,
        service,
        username,
        expected_password,
        expect_password_found,
        case(
            Vec::new(),
            "test_service",
            Some("test_username"),
            None,
            false
        ),
        case(
            vec![
                PasswordEntry::new(
                    "test_service_1".to_string(),
                    Some("test_username_1".to_string()),
                    "test_password_1".to_string(),
                ),
            ],
            "test_service_1",
            Some("test_username_1"),
            Some(PasswordEntry::new(
                "test_service_1".to_string(),
                Some("test_username_1".to_string()),
                "test_password_1".to_string(),
            )),
            true
        ),
        case(
            vec![
                PasswordEntry::new(
                    "test_service_1".to_string(),
                    Some("test_username_1".to_string()),
                    "test_password_1".to_string(),
                ),
                PasswordEntry::new(
                    "test_service_2".to_string(),
                    Some("test_username_2".to_string()),
                    "test_password_2".to_string(),
                ),
            ],
            "test_service_2",
            Some("test_username_2"),
            Some(PasswordEntry::new(
                "test_service_2".to_string(),
                Some("test_username_2".to_string()),
                "test_password_2".to_string(),
            )),
            true
        ),
    )]
    fn test_find_password(
        test_passwords: Vec<PasswordEntry>,
        service: &str,
        username: Option<&str>,
        expected_password: Option<PasswordEntry>,
        expect_password_found: bool,
    ) {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut store =
            PasswordStore::new(temp_file_name.to_string(), TEST_MASTER_PASSWORD.to_string())
                .unwrap()
                .load_passwords()
                .unwrap();
        store.passwords = Some(test_passwords.into());
        let found_password =
            store.find_password(service.to_string(), username.map(|u| u.to_string()));
        assert_eq!(found_password.is_some(), expect_password_found);
        if let Some(found_password) = found_password {
            assert_eq!(found_password.service, service);
            if let Some(username) = username {
                assert_eq!(found_password.username.as_deref(), Some(username));
            } else {
                assert_eq!(found_password.username, None);
            }
        }
        if let Some(expected_password) = expected_password {
            assert_eq!(found_password, Some(&expected_password));
        } else {
            assert_eq!(found_password, None);
        }
    }

    struct PasswordEntryWrapper {
        password_entry: PasswordEntry,
    }

    impl PasswordEntryWrapper {
        fn new(password_entry: PasswordEntry) -> Self {
            PasswordEntryWrapper { password_entry }
        }

        fn password(&self) -> &str {
            self.password_entry.password()
        }
    }

    #[test]
    fn test_add_password() {
        // Create a new PasswordStore instance with a temporary file
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut store =
            PasswordStore::new(temp_file_name.to_string(), TEST_MASTER_PASSWORD.to_string())
                .unwrap()
                .load_passwords() // Load existing passwords
                .unwrap();

        // Create a sample password to add
        let password = PasswordEntry::new(
            "service1".to_string(),
            Some("user1".to_string()),
            "password1".to_string(),
        );

        // Add the password to the store
        store = store
            .add_password(
                password.service.clone(),
                password.username.clone(),
                password.password().to_string(), // Use the password method to access the password
            )
            .unwrap();

        // Check if the password was added correctly
        assert!(store.passwords.is_some());

        let stored_passwords = store.passwords.unwrap();

        let found_password = stored_passwords
            .find(password.service.clone(), password.username.clone())
            .unwrap();
        let password_wrapper = PasswordEntryWrapper::new(found_password.clone());

        let found_password = stored_passwords
            .find(password.service.clone(), password.username.clone())
            .unwrap();
        assert_eq!(password.service, found_password.service);
        assert_eq!(password.username, found_password.username);
        assert_eq!(password.password(), password_wrapper.password()); // Use the password method to access the password
    }
}
