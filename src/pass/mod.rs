use colored::*;
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    pub fn print_password(
        &self,
        color: Option<Color>,
        writer: &mut dyn Write,
    ) -> anyhow::Result<()> {
        if let Some(color) = color {
            writeln!(writer, "Password: {}", self.password.color(color))?;
        } else {
            writeln!(writer, "Password: {}", self.password)?;
        }
        Ok(())
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

    pub fn print_all(
        &self,
        show_passwords: bool,
        color: Option<Color>,
        writer: &mut dyn Write,
    ) -> anyhow::Result<()> {
        for pwd in self.0.iter() {
            if show_passwords {
                if let Some(username) = &pwd.username {
                    if let Some(color) = color {
                        writeln!(
                            writer,
                            "Service: {}, Username: {}, Password: {}",
                            pwd.service.color(color),
                            username.color(color),
                            pwd.password.color(color)
                        )?;
                    } else {
                        writeln!(
                            writer,
                            "Service: {}, Username: {}, Password: {}",
                            pwd.service, username, pwd.password
                        )?;
                    }
                } else if let Some(color) = color {
                    writeln!(
                        writer,
                        "Service: {}, Password: {}",
                        pwd.service.color(color),
                        pwd.password.color(color)
                    )?;
                } else {
                    writeln!(
                        writer,
                        "Service: {}, Password: {}",
                        pwd.service, pwd.password
                    )?;
                }
            } else if let Some(username) = &pwd.username {
                if let Some(color) = color {
                    writeln!(
                        writer,
                        "Service: {}, Username: {}, Password: {}",
                        pwd.service.color(color),
                        username.color(color),
                        "***".color(color)
                    )?;
                } else {
                    writeln!(
                        writer,
                        "Service: {}, Username: {}, Password: ***",
                        pwd.service, username
                    )?;
                }
            } else if let Some(color) = color {
                writeln!(
                    writer,
                    "Service: {}, Password: {}",
                    pwd.service.color(color),
                    "***".color(color)
                )?;
            } else {
                writeln!(writer, "Service: {}, Password: ***", pwd.service)?;
            }
        }
        Ok(())
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
        expected_output,
        case(vec![], true, ""),
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
        "Service: service1, Username: username1, Password: password1\nService: service2, Username: username2, Password: password2\n"
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
        "Service: service1, Password: password1\nService: service2, Username: username2, Password: password2\n"
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
        "Service: service1, Password: password1\nService: service2, Password: password2\n"
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
        "Service: service1, Username: username1, Password: ***\nService: service2, Username: username2, Password: ***\n"
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
        "Service: service1, Password: ***\nService: service2, Username: username2, Password: ***\n"
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
        "Service: service1, Password: ***\nService: service2, Password: ***\n"
        ),
    )]
    fn test_print_all(
        test_passwords: Vec<PasswordEntry>,
        show_passwords: bool,
        expected_output: &str,
    ) {
        let passwords = Passwords::from(test_passwords);
        let mut output = Vec::new();
        passwords
            .print_all(show_passwords, None, &mut output)
            .unwrap();
        let output_str = String::from_utf8(output).unwrap();
        print!("OUTPUT STR: {} END OUTPUT", output_str);
        assert!(output_str.contains(expected_output));
    }
}
