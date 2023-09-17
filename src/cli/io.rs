use crossterm::style::{style, Attribute, Color, Stylize};
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
            colorize(">> ", MessageType::DarkYellow)
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
    write!(writer, "{}", colorize(">> ", MessageType::DarkYellow))
        .unwrap_or_else(|_| print!("{}", colorize(">> ", MessageType::DarkYellow)));
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
    DarkRed,
    DarkYellow,
}

impl MessageType {
    pub fn get_color(self) -> Color {
        match self {
            Self::Success => Color::Green,
            Self::Error => Color::Red,
            Self::Warning => Color::Yellow,
            Self::Info => Color::Blue,
            Self::DarkRed => Color::DarkRed,
            Self::DarkYellow => Color::DarkYellow,
        }
    }
}

pub fn colorize(message: &str, message_type: MessageType) -> String {
    style(message).with(message_type.get_color()).to_string()
}

pub fn bold(message: &str) -> String {
    style(message).attribute(Attribute::Bold).to_string()
}

pub fn print<W: Write>(writer: &mut W, message: &str, message_type: Option<MessageType>) {
    let message = match message_type {
        Some(message_type) => colorize(message, message_type),
        None => String::from(message),
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
        Some(message_type) => colorize(key, message_type),
        None => String::from(key),
    };
    let colored_value = match value_message_type {
        Some(message_type) => colorize(value, message_type),
        None => String::from(value),
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
            format!("test prompt\n{}", colorize(">> ", MessageType::DarkYellow))
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
                colorize(">> ", MessageType::DarkYellow)
            )))
            .times(1)
            .returning(|_| Ok("secret".to_string()));

        let input = read_hidden_input("password", &mock_prompt_password);
        assert_eq!(input, "secret");
    }

    use std::io::Cursor;

    #[test]
    fn test_colorize() {
        let test_message = "test_message";
        let colored_message = colorize(test_message, MessageType::Success);
        assert!(colored_message.contains(test_message));
        assert!(colored_message.len() > test_message.len());
    }

    #[test]
    fn test_bold() {
        let test_message = "test_message";
        let bold_message = bold(test_message);
        assert!(bold_message.contains(test_message));
        assert!(bold_message.len() > test_message.len());
    }

    #[test]
    fn test_print() {
        let mut output = Cursor::new(vec![]);
        let test_message = "test_message";
        print(&mut output, test_message, Some(MessageType::Success));
        let output_str = String::from_utf8(output.into_inner()).unwrap();
        assert!(output_str.contains(test_message));
        assert!(output_str.len() > test_message.len());
    }

    #[test]
    fn test_print_key_value_with_color() {
        let mut output = Cursor::new(vec![]);
        let key = "key";
        let value = "value";
        print_key_value_with_color(
            &mut output,
            key,
            value,
            Some(MessageType::Success),
            Some(MessageType::Error),
            None,
        );
        let output_str = String::from_utf8(output.into_inner()).unwrap();
        assert!(output_str.contains(key));
        assert!(output_str.contains(value));
    }
}
