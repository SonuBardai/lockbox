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
        rpassword::prompt_password(prompt)
    }
}

pub fn read_hidden_input(prompt: &str, prompt_password: &dyn PromptPassword) -> String {
    let input = prompt_password
        .prompt_password(format!(
            "Please enter the {prompt}\n{}",
            colorize(">> ", MessageType::Warning)
        ))
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
    write!(writer, "{}", colorize(">> ", MessageType::Warning))
        .unwrap_or_else(|_| print!("{}", colorize(">> ", MessageType::Warning)));
    stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_owned()
}

#[derive(Clone, Copy)]
pub enum MessageType {
    Success,
    Error,
    Warning,
    Info,
    BrightRed,
}

impl MessageType {
    pub fn get_color(self) -> Color {
        match self {
            Self::Success => Color::Green,
            Self::Error => Color::Red,
            Self::Warning => Color::Yellow,
            Self::Info => Color::Blue,
            Self::BrightRed => Color::BrightRed,
        }
    }
}

pub fn colorize(message: &str, message_type: MessageType) -> ColoredString {
    message.color(message_type.get_color())
}

pub fn bold(message: &str) -> ColoredString {
    message.bold()
}

pub fn print<W: Write>(writer: &mut W, message: &str, message_type: Option<MessageType>) {
    let message = match message_type {
        Some(message_type) => message.color(message_type.get_color()),
        None => message.normal(),
    };
    writeln!(writer, "{message}").unwrap_or_else(|_| println!("{message}"));
}

pub fn print_key_value_with_color<W: Write>(
    writer: &mut W,
    key: &str,
    value: &str,
    key_message_type: Option<MessageType>,
    value_message_type: Option<MessageType>,
    end: Option<&str>,
) {
    let colored_key = match key_message_type {
        Some(message_type) => key.color(message_type.get_color()),
        None => key.normal(),
    };
    let colored_value = match value_message_type {
        Some(message_type) => value.color(message_type.get_color()),
        None => value.normal(),
    };
    let end = end.unwrap_or("\n");
    write!(writer, "{}: {}{}", colored_key, colored_value, end)
        .unwrap_or_else(|_| println!("{}: {}{}", colored_key, colored_value, end));
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
            format!("test prompt\n{}", colorize(">> ", MessageType::Warning))
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
                colorize(">> ", MessageType::Warning)
            )))
            .times(1)
            .returning(|_| Ok("secret".to_string()));

        let input = read_hidden_input("password", &mock_prompt_password);
        assert_eq!(input, "secret");
    }
}
