use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::cli::io::{print, print_key_value_with_color, MessageType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PasswordEntry {
    pub service: String,
    pub username: Option<String>,
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
    pub fn print_password<W: Write>(&self, writer: &mut W, message_type: Option<MessageType>) {
        print_key_value_with_color(writer, "Password", &self.password, None, message_type, None);
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Passwords(Vec<PasswordEntry>);

impl Default for Passwords {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<PasswordEntry>> for Passwords {
    fn from(passwords: Vec<PasswordEntry>) -> Self {
        Passwords(passwords)
    }
}

impl Passwords {
    pub fn new() -> Self {
        Passwords(vec![])
    }

    pub fn append(&mut self, new_password: PasswordEntry) {
        self.0.push(new_password);
    }

    pub fn find(&self, service: String, username: Option<String>) -> Option<&PasswordEntry> {
        self.0
            .iter()
            .find(|pwd| pwd.service == service && pwd.username == username)
    }

    pub fn remove(&mut self, service: String, username: Option<String>) -> Option<PasswordEntry> {
        if let Some(index) = self
            .0
            .iter()
            .position(|pwd| pwd.service == service && pwd.username == username)
        {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    pub fn parse_passwords(raw_passwords: &str) -> Result<Passwords, anyhow::Error> {
        let passwords: Passwords = serde_json::from_str(raw_passwords)?;
        Ok(passwords)
    }

    pub fn print_all<W: Write>(
        &self,
        writer: &mut W,
        show_passwords: bool,
        message_type: Option<MessageType>,
    ) {
        if !self.0.is_empty() {
            for pwd in self.0.iter() {
                print_key_value_with_color(
                    writer,
                    "Service",
                    &pwd.service,
                    None,
                    message_type,
                    Some(","),
                );
                if pwd.username.is_some() {
                    print_key_value_with_color(
                        writer,
                        "Username",
                        pwd.username.as_ref().unwrap(),
                        None,
                        message_type,
                        Some(","),
                    );
                }
                if show_passwords {
                    print_key_value_with_color(
                        writer,
                        "Password",
                        &pwd.password,
                        None,
                        message_type,
                        None,
                    );
                } else {
                    print_key_value_with_color(writer, "Password", "***", None, message_type, None);
                }
            }
        } else {
            print(writer, "No passwords found!", Some(MessageType::Warning));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_passwords() {
        assert_eq!(Passwords::new(), Passwords(vec![]));
        assert_eq!(Passwords::default(), Passwords(vec![]));
    }

    #[rstest(
        test_passwords,
        show_passwords,
        case(vec![], true),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                Some("username1".to_string()),
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                Some("username2".to_string()),
                "password2".to_string(),
            ),
        ],
        true,
        ),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                None,
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                Some("username2".to_string()),
                "password2".to_string(),
            ),
        ],
        true,
        ),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                None,
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                None,
                "password2".to_string(),
            ),
        ],
        true,
        ),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                Some("username1".to_string()),
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                Some("username2".to_string()),
                "password2".to_string(),
            ),
        ],
        false,
        ),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                None,
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                Some("username2".to_string()),
                "password2".to_string(),
            ),
        ],
        false,
        ),
        case(vec![
            PasswordEntry::new(
                "service1".to_string(),
                None,
                "password1".to_string(),
            ),
            PasswordEntry::new(
                "service2".to_string(),
                None,
                "password2".to_string(),
            ),
        ],
        false,
        ),
    )]
    fn test_print_all(test_passwords: Vec<PasswordEntry>, show_passwords: bool) {
        let passwords = Passwords::from(test_passwords);
        let mut output = Vec::new();
        passwords.print_all(&mut output, show_passwords, None);
        let output_str = String::from_utf8(output).unwrap();
        for password in passwords.0 {
            if show_passwords {
                assert!(output_str.contains(&password.password))
            } else {
                assert!(output_str.contains("***"))
            };
            assert!(output_str.contains(&password.service));
            if password.username.is_some() {
                assert!(output_str.contains(&password.username.unwrap()))
            };
        }
    }
}
