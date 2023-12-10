use crate::{
    cli::{
        args::{get_password_store_path, Length, DEFAULT_PASSWORD_FILENAME},
        commands::{
            add_password, generate_password, list_passwords, remove_password, show_password,
            update_master_password,
        },
        io::{
            bold, colorize, print, read_hidden_input, read_hidden_input_with_confirmation,
            read_terminal_input, MessageType, PromptPassword,
        },
    },
    store::PasswordStore,
};
use passwords::PasswordGenerator;
use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

pub fn repl<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_password: &dyn PromptPassword,
    file_name: String,
) {
    print(writer, &bold("Welcome to L🦀CKBOX!\n"), None);
    let file_path =
        get_password_store_path(file_name).unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
    let master: String = if !file_path.exists() {
        read_hidden_input_with_confirmation(writer, "master password", prompt_password)
    } else {
        read_hidden_input("master password", prompt_password)
    };
    let password_store = match PasswordStore::new(file_path, master) {
        Ok(password_store) => password_store,
        Err(err) => {
            writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
            return;
        }
    };
    run_repl(reader, writer, prompt_password, password_store);
}

pub fn run_repl<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_password: &dyn PromptPassword,
    mut password_store: PasswordStore,
) {
    while let Err(err) = password_store.load() {
        print(
            writer,
            &format!("Failed to load password store: {err}"),
            Some(MessageType::Error),
        );
        let master = read_hidden_input("master password", prompt_password);
        password_store.update_master(master);
    }
    loop {
        let message = [
            format!(
                "[{}] {} password",
                colorize(&bold("1"), MessageType::Success),
                colorize(&bold("add"), MessageType::Success)
            ),
            format!(
                "[{}] {} random password",
                colorize(&bold("2"), MessageType::Success),
                colorize(&bold("generate"), MessageType::Success)
            ),
            format!(
                "[{}] {} passwords",
                colorize(&bold("3"), MessageType::Success),
                colorize(&bold("list"), MessageType::Success)
            ),
            format!(
                "[{}] {} password",
                colorize(&bold("4"), MessageType::Success),
                colorize(&bold("remove"), MessageType::Success)
            ),
            format!(
                "[{}] {} password",
                colorize(&bold("5"), MessageType::Success),
                colorize(&bold("show"), MessageType::Success)
            ),
            format!(
                "[{}] {} password",
                colorize(&bold("6"), MessageType::Success),
                colorize(&bold("update master"), MessageType::Success)
            ),
            format!(
                "[{}] {}",
                colorize(&bold("7"), MessageType::Success),
                colorize(&bold("exit"), MessageType::Success)
            ),
        ];

        let message = message.join(" ");
        writeln!(writer, "\nEnter {message}").unwrap();
        let input = read_terminal_input(reader, writer, None);
        match input.as_str() {
            "1" | "add" | "a" => {
                handle_add_password(reader, writer, prompt_password, &mut password_store)
            }
            "2" | "generate" | "g" => handle_generate_password(writer),
            "3" | "list" | "l" => handle_list_passwords(writer, &mut password_store),
            "4" | "remove" | "r" => handle_remove_password(reader, writer, &mut password_store),
            "5" | "show" | "s" => handle_show_password(reader, writer, &mut password_store),
            "6" | "update" | "u" => {
                handle_update_master_password(writer, prompt_password, &mut password_store)
            }
            _ => break,
        }
    }
}

fn handle_add_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_password: &dyn PromptPassword,
    password_store: &mut PasswordStore,
) {
    let message = [
        format!(
            "[{}] {} random password",
            colorize(&bold("1"), MessageType::Success),
            colorize(&bold("generate"), MessageType::Success)
        ),
        format!(
            "[{}] {} your own password",
            colorize(&bold("2"), MessageType::Success),
            colorize(&bold("enter"), MessageType::Success)
        ),
        format!(
            "[{}] {}",
            colorize(&bold("3"), MessageType::Success),
            colorize(&bold("cancel"), MessageType::Success)
        ),
    ];
    let message = message.join(" ");
    writeln!(writer, "{}", message).unwrap();
    let input = read_terminal_input(reader, writer, None);
    let generate = match input.as_str() {
        "1" | "generate" | "g" | "random" | "r" => true,
        "2" | "enter" | "e" => false,
        _ => return,
    };
    let service = read_terminal_input(reader, writer, Some("Please enter the service name"));
    let username =
        read_terminal_input(reader, writer, Some("Please enter the username (Optional)"));
    let username = Option::from(username).filter(|s| !s.is_empty());
    let password_generator = PasswordGenerator::new()
        .length(Length::Sixteen.get_val())
        .lowercase_letters(true)
        .uppercase_letters(true)
        .numbers(true)
        .symbols(false)
        .strict(true);
    match add_password(
        writer,
        prompt_password,
        password_store,
        service,
        username,
        None,
        generate,
        password_generator,
    ) {
        Ok(_) => print(
            writer,
            "Password added successfully",
            Some(MessageType::Success),
        ),
        Err(err) => print(writer, &format!("Error: {err}"), Some(MessageType::Error)),
    };
}

fn handle_generate_password<W: Write>(writer: &mut W) {
    match generate_password(writer, Length::Sixteen, false, true, true, true, 1) {
        Ok(_) => (),
        Err(err) => print(writer, &format!("Error: {err}"), Some(MessageType::Error)),
    };
}

fn handle_list_passwords<W: Write>(writer: &mut W, password_store: &mut PasswordStore) {
    list_passwords(writer, password_store, true).unwrap_or_else(|err| {
        print(
            writer,
            &format!("Failed to load passwords to store: {err}"),
            Some(MessageType::Error),
        )
    });
}

fn handle_remove_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    password_store: &mut PasswordStore,
) {
    let service = read_terminal_input(reader, writer, Some("Please enter the service name"));
    let username =
        read_terminal_input(reader, writer, Some("Please enter the username (Optional)"));
    let username = Option::from(username).filter(|s| !s.is_empty());
    remove_password(writer, password_store, service, username).unwrap_or_else(|err| {
        print(
            writer,
            &format!("Failed to remove password: {err}"),
            Some(MessageType::Error),
        )
    })
}

fn handle_show_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    password_store: &mut PasswordStore,
) {
    let service = read_terminal_input(reader, writer, Some("Please enter the service name"));
    let username =
        read_terminal_input(reader, writer, Some("Please enter the username (Optional)"));
    let username = Option::from(username).filter(|s| !s.is_empty());
    if show_password(writer, password_store, service, username).is_err() {
        print(writer, "Password not found", None);
    };
}

fn handle_update_master_password<W: Write>(
    writer: &mut W,
    prompt_password: &dyn PromptPassword,
    password_store: &mut PasswordStore,
) {
    let new_master_password =
        read_hidden_input_with_confirmation(writer, "master password", prompt_password);
    update_master_password(writer, new_master_password, password_store).unwrap_or_else(|err| {
        print(
            writer,
            &format!("Failed to update master password: {err}"),
            Some(MessageType::Error),
        );
    });
}

#[cfg(test)]
mod tests {
    use crate::cli::io::{colorize, MockPromptPassword};

    use super::*;

    use mockall::predicate::eq;
    use tempfile::NamedTempFile;

    use rstest::rstest;

    #[test]
    fn test_repl() {
        let mut input = b"" as &[u8];
        let mut output = Vec::new();
        let mut mock_prompt_password = MockPromptPassword::new();
        mock_prompt_password
            .expect_prompt_password()
            .with(eq(format!(
                "Please enter the master password\n{}",
                colorize(">> ", MessageType::DarkYellow)
            )))
            .times(1)
            .returning(|_| Ok("secret".to_string()));
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap().to_string();

        repl(
            &mut input,
            &mut output,
            &mock_prompt_password,
            temp_file_name,
        );

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Welcome to L🦀CKBOX!"));
        let operations = [
            "add",
            "random",
            "list",
            "remove",
            "show",
            "update master",
            "exit",
        ];
        for operation in operations {
            assert!(output_str.contains(operation))
        }
    }

    #[rstest(
        input,
        expected_output,
        case(
            b"add\n1\ntest_service\ntest_username\n7\n" as &[u8],
            vec![
                "generate", "enter", "cancel", "Please enter the service name", "Please enter the username (Optional)", "Password added successfully", ">>",
            ],
        ),
        case(
            b"list\nexit\n" as &[u8],
            vec!["Service:", "service",  "Username:", "username", "Password:", "password"]
        ),
        case(
            b"generate\nexit\n" as &[u8],
            vec!["Random password generated."],
        ),
        case(
            b"remove\nservice\nusername\nexit\n" as &[u8],
            vec!["Password deleted"],
        ),
        case(
            b"show\nservice\nusername\nexit\n" as &[u8],
            vec!["Password:", "password"],
        ),
    )]
    fn test_run_repl(input: &[u8], expected_output: Vec<&str>) {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut writer = std::io::Cursor::new(Vec::new());
        let mock_prompt_password = &MockPromptPassword::new();
        add_password(
            &mut writer,
            mock_prompt_password,
            &mut password_store,
            "service".to_string(),
            Some("username".to_string()),
            Some("password".to_string()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();
        let mut input = input;
        let mut output = Vec::new();
        let mock_prompt_password = &MockPromptPassword::new();
        run_repl(
            &mut input,
            &mut output,
            mock_prompt_password,
            password_store,
        );

        let output_str = String::from_utf8(output).unwrap();
        let operations = [
            "add",
            "random",
            "list",
            "remove",
            "show",
            "update master",
            "exit",
        ];
        for operation in operations {
            assert!(output_str.contains(operation));
        }
        for expected in expected_output {
            assert!(output_str.contains(expected));
        }
    }

    #[test]
    fn test_handle_add_password() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut input = b"1\ntest_service\ntest_username\n" as &[u8];
        let mut output = Vec::new();
        let mock_prompt_password = &MockPromptPassword::new();

        handle_add_password(
            &mut input,
            &mut output,
            mock_prompt_password,
            &mut password_store,
        );

        let output_str = String::from_utf8(output).unwrap();
        let operations = ["random password", "enter", "cancel"];
        for operation in operations {
            assert!(output_str.contains(operation))
        }
        assert!(output_str.contains("Please enter the service name",));
        assert!(output_str.contains("Please enter the username (Optional)",));
        assert!(output_str.contains("Password added successfully"));
    }

    #[test]
    fn test_handle_generate_password() {
        let mut output = Vec::new();

        handle_generate_password(&mut output);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Random password generated."));
    }

    #[test]
    fn test_handle_list_passwords() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut writer = std::io::Cursor::new(Vec::new());
        let mock_prompt_password = &MockPromptPassword::new();
        add_password(
            &mut writer,
            mock_prompt_password,
            &mut password_store,
            "service".to_string(),
            Some("username".to_string()),
            Some("password".to_string()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();
        let mut output = Vec::new();

        handle_list_passwords(&mut output, &mut password_store);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("service"));
        assert!(output_str.contains("username"));
        assert!(output_str.contains("password"));
    }

    #[test]
    fn test_handle_remove_password() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut writer = std::io::Cursor::new(Vec::new());
        let mock_prompt_password = &MockPromptPassword::new();
        add_password(
            &mut writer,
            mock_prompt_password,
            &mut password_store,
            "service".to_string(),
            Some("username".to_string()),
            Some("password".to_string()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();

        let mut input = b"test_service\ntest_username\n" as &[u8];
        let mut output = Vec::new();
        handle_remove_password(&mut input, &mut output, &mut password_store);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Password not found"));

        input = b"service\nusername\n" as &[u8];
        output = Vec::new();
        handle_remove_password(&mut input, &mut output, &mut password_store);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Password deleted"));
    }

    #[test]
    fn test_handle_show_password() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut writer = std::io::Cursor::new(Vec::new());
        let mock_prompt_password = &MockPromptPassword::new();
        add_password(
            &mut writer,
            mock_prompt_password,
            &mut password_store,
            "service".to_string(),
            Some("username".to_string()),
            Some("password".to_string()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();

        let mut input = b"test_service\ntest_username\n" as &[u8];
        let mut output = Vec::new();
        handle_show_password(&mut input, &mut output, &mut password_store);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Password not found"));

        input = b"service\nusername\n" as &[u8];
        output = Vec::new();
        handle_show_password(&mut input, &mut output, &mut password_store);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("password"));
    }

    #[test]
    fn test_handle_update_master_password() {
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut password_store = PasswordStore::new(temp_file, "secret".to_string()).unwrap();
        let mut writer = Vec::new();
        let mut mock_prompt_password = MockPromptPassword::new();
        mock_prompt_password
            .expect_prompt_password()
            .returning(|_| Ok("secret".to_string()));
        mock_prompt_password
            .expect_prompt_password()
            .returning(|_| Ok("newmasterpassword".to_string()));
        handle_update_master_password(&mut writer, &mock_prompt_password, &mut password_store);
        let output_str = String::from_utf8(writer).unwrap();
        assert!(output_str.contains(&colorize(
            "Master password updated successfully",
            MessageType::Success
        )));
    }
}
