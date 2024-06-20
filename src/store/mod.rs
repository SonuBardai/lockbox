use crate::cli::io::{print, MessageType};
use crate::pass::PasswordEntry;
use crate::{
    crypto::{encrypt_contents, get_cipher, get_random_salt},
    pass::Passwords,
};
use aes_gcm::aead::Aead;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const EMPTY_PASSWORDS: &str = "[]";

pub struct PasswordStore {
    pub file_path: PathBuf,
    master_password: String,
    passwords: Option<Passwords>,
}

impl PasswordStore {
    pub fn new(file_path: PathBuf, master_password: String) -> anyhow::Result<Self> {
        if !file_path.exists() || fs::metadata(&file_path)?.len() == 0 {
            let salt = get_random_salt();
            let (empty_json, nonce) = encrypt_contents(EMPTY_PASSWORDS, &master_password, &salt);
            let mut content = salt.to_vec();
            content.extend(nonce);
            content.extend(empty_json);
            fs::write(&file_path, content)?;
        }
        let store = Self {
            file_path,
            master_password,
            passwords: None,
        };
        Ok(store)
    }

    pub fn load(&mut self) -> anyhow::Result<&mut Self> {
        let encrypted_file = std::fs::read(&self.file_path)?;
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

    pub fn dump(&mut self) -> anyhow::Result<&mut Self> {
        let encrypted_file = std::fs::read(&self.file_path)?;
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
        std::fs::write(&self.file_path, content)?;
        Ok(self)
    }

    pub fn push(
        &mut self,
        service: String,
        username: Option<String>,
        password: String,
    ) -> anyhow::Result<&mut Self> {
        let new_password = PasswordEntry::new(service, username, password);
        if let Some(ref mut passwords) = self.passwords {
            passwords.append(new_password);
        } else {
            panic!("Load passwords before appending")
        }
        Ok(self)
    }

    pub fn pop<W: Write>(
        &mut self,
        writer: &mut W,
        service: String,
        username: Option<String>,
    ) -> &mut Self {
        if let Some(_password) = self
            .passwords
            .as_mut()
            .and_then(|passwords| passwords.remove(service, username))
        {
            print(writer, "Password deleted", Some(MessageType::Success));
        } else {
            print(writer, "Password not found", Some(MessageType::Warning));
        }
        self
    }

    pub fn find(&self, service: String, username: Option<String>) -> Option<&PasswordEntry> {
        self.passwords
            .as_ref()
            .and_then(|passwords| passwords.find(service, username))
    }

    pub fn print<W: Write>(
        &self,
        writer: &mut W,
        show_passwords: bool,
        message_type: Option<MessageType>,
    ) {
        if let Some(passwords) = self.passwords.as_ref() {
            passwords.print_all(writer, show_passwords, message_type)
        }
    }

    pub fn update_master(&mut self, new_master_password: String) -> &mut Self {
        self.master_password = new_master_password;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{commands::add_password, io::MockPromptPassword};
    use passwords::PasswordGenerator;
    use rstest::rstest;
    use tempfile::NamedTempFile;

    use super::*;
    const TEST_MASTER_PASSWORD: &str = "test_master";

    #[test]
    fn test_new_password_store() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let store =
            PasswordStore::new(temp_file.clone(), TEST_MASTER_PASSWORD.to_string()).unwrap();
        assert_eq!(store.file_path, temp_file);
        assert_eq!(store.master_password, TEST_MASTER_PASSWORD);
        assert!(store.passwords.is_none());
        assert!(temp_file.exists());
    }

    #[test]
    fn test_new_password_store_with_nonexistent_file() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let store =
            PasswordStore::new(temp_file.clone(), TEST_MASTER_PASSWORD.to_string()).unwrap();
        assert_eq!(store.file_path, temp_file);
        assert_eq!(store.master_password, TEST_MASTER_PASSWORD);
        assert!(store.passwords.is_none());
        assert!(PathBuf::from(temp_file.to_str().unwrap()).exists());
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
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut store = PasswordStore::new(temp_file, TEST_MASTER_PASSWORD.to_string()).unwrap();
        store.load().unwrap();
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
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut store = PasswordStore::new(temp_file, TEST_MASTER_PASSWORD.to_string()).unwrap();
        store.load().unwrap();
        store.passwords = Some(test_passwords.into());
        let found_password = store.find(service.to_string(), username.map(|u| u.to_string()));
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

    #[rstest(
        show_passwords,
        passwords,
        expected_output,
        case(
            true,
            vec![("service1", Some("username1"), "password1"), ("service2", None, "password2")],
            vec!["Service:", "service1", "Username:", "username", "Password:", "password"]
        ),
        case(
            false,
            vec![("service1", Some("username1"), "password1"), ("service2", None, "password2")],
            vec!["Service:", "service1", "Username:", "username", "Password:", "***"]
        )
    )]
    fn test_print(
        show_passwords: bool,
        passwords: Vec<(&str, Option<&str>, &str)>,
        expected_output: Vec<&str>,
    ) {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store =
            PasswordStore::new(temp_file, "master_password".to_string()).unwrap();
        let mut writer = std::io::Cursor::new(Vec::new());
        let mock_prompt_password = &MockPromptPassword::new();
        passwords
            .into_iter()
            .for_each(|(service, username, password)| {
                add_password(
                    &mut writer,
                    mock_prompt_password,
                    &mut password_store,
                    service.to_string(),
                    username.map(|u| u.to_string()),
                    Some(password.to_string()),
                    false,
                    PasswordGenerator::default(),
                )
                .unwrap()
            });

        let mut output = Vec::new();
        let mut writer = std::io::Cursor::new(output);
        password_store.print(&mut writer, show_passwords, Some(MessageType::Info));

        output = writer.into_inner();
        let output_str = String::from_utf8(output).unwrap();
        for item in expected_output {
            assert!(output_str.contains(item));
        }
    }

    #[test]
    fn test_update_master() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store =
            PasswordStore::new(temp_file, "some_master_password".to_string()).unwrap();
        password_store.update_master("new_master_password".to_string());
        assert!(password_store.master_password == "new_master_password");
        assert!(password_store.load().is_err());
        if let Err(err) = password_store.load() {
            err.to_string()
                .contains("Master password incorrect. Please try again.");
        };
    }

    #[test]
    fn test_push_without_loading() {
        // Test pushing a password entry without loading passwords first.
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut store =
            PasswordStore::new(temp_file.clone(), TEST_MASTER_PASSWORD.to_string()).unwrap();

        // Attempt to push a password entry without loading.
        let result = store.push(
            "test_service".to_string(),
            Some("test_username".to_string()),
            "test_password".to_string(),
        );

        assert!(result.is_err());
        assert!(store.passwords.is_none());
    }

    #[test]
    fn test_pop_nonexistent_password() {
        // Test popping a password that doesn't exist.
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut store =
            PasswordStore::new(temp_file.clone(), TEST_MASTER_PASSWORD.to_string()).unwrap();
        store.load().unwrap();

        // Attempt to pop a non-existent password.
        let mut output = Vec::new();
        store.pop(&mut output, "nonexistent_service".to_string(), None);
        let output_str = String::from_utf8(output).unwrap();

        assert!(output_str.contains("Password not found"));
    }

    #[test]
    fn test_dump_empty_passwords() {
        // Test dumping an empty password store.
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut store =
            PasswordStore::new(temp_file.clone(), TEST_MASTER_PASSWORD.to_string()).unwrap();
        store.load().unwrap();

        // Dump the empty password store.
        let result = store.dump();

        assert!(result.is_ok());
    }
}


