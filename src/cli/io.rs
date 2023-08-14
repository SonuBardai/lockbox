use colored::*;
use std::io::{stdout, BufRead, Error, Write};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PromptPassword {
    fn prompt_password(&self, prompt: String) -> Result<String, std::io::Error>;
}

pub struct RpasswordPromptPassword;

impl PromptPassword for RpasswordPromptPassword {
    fn prompt_password(&self, prompt: String) -> Result<String, Error> {
        rpassword::prompt_password(&prompt)
    }
}

pub fn read_hidden_input(prompt: &str, prompt_password: &dyn PromptPassword) -> String {
    let input = prompt_password
        .prompt_password(format!("Please enter the {}\n{}", prompt, ">> ".yellow()))
        .unwrap_or_else(|_| panic!("Failed to read {}", prompt));
    input.trim().to_string()
}

pub fn read_terminal_input<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: Option<&str>,
) -> String {
    if let Some(prompt) = prompt {
        writeln!(writer, "{}", prompt).unwrap();
    }
    write!(writer, "{}", ">> ".yellow()).unwrap();
    stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn test_read_terminal_input() {
        let mut input = b"test input\n" as &[u8];
        let mut output = Vec::new();
        let result = read_terminal_input(&mut input, &mut output, Some("test prompt"));
        assert_eq!(result, "test input");
        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("test prompt\n{}", ">> ".yellow())
        );
    }

    #[test]
    fn test_read_hidden_input() {
        let mut mock_prompt_password = MockPromptPassword::new();
        mock_prompt_password
            .expect_prompt_password()
            .with(eq(format!(
                "Please enter the {}\n{}",
                "password",
                ">> ".yellow()
            )))
            .times(1)
            .returning(|_| Ok("secret".to_string()));

        let input = read_hidden_input("password", &mock_prompt_password);
        assert_eq!(input, "secret");
    }
}
