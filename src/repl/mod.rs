use crate::{
    cli::{
        args::Length,
        commands::{
            add_password, generate_password, list_passwords, remove_password, show_password,
        },
        io::{read_hidden_input, read_terminal_input, PromptPassword, RpasswordPromptPassword},
    },
    store::PasswordStore,
};
use colored::*;
use passwords::PasswordGenerator;
use std::io::{stdin, stdout, BufRead, Write};

pub fn repl<W: Write>(file_name: String, writer: &mut W, prompt_password: &dyn PromptPassword) {
    writeln!(writer, "{}", "Welcome to L🦀CKBOX!\n".bold()).unwrap();
    let master = read_hidden_input("master password", prompt_password);
    let password_store = match PasswordStore::new(file_name, master) {
        Ok(password_store) => password_store,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    run_repl(password_store);
}

pub fn run_repl(mut password_store: PasswordStore) {
    while let Err(err) = password_store.load() {
        eprintln!("{}: {err}", "Failed to load password store".red());
        let master = read_hidden_input("master password", &RpasswordPromptPassword);
        password_store.update_master(master);
    }
    loop {
        let message = [
            format!("[{}] {} password", "1".green().bold(), "add".green().bold()),
            format!(
                "[{}] {} random password",
                "2".green().bold(),
                "generate".green().bold()
            ),
            format!(
                "[{}] {} passwords",
                "3".green().bold(),
                "list".green().bold()
            ),
            format!(
                "[{}] {} password",
                "4".green().bold(),
                "remove".green().bold()
            ),
            format!(
                "[{}] {} password",
                "5".green().bold(),
                "show".green().bold()
            ),
            format!("[{}] {}", "6".green().bold(), "exit".green().bold()),
        ];
        let message = message.join(" ");
        println!("\nEnter {message}");
        let input = read_terminal_input(&mut stdin().lock(), &mut stdout().lock(), None);
        match input.as_str() {
            "1" | "add" | "a" => handle_add_password(
                &mut stdin().lock(),
                &mut stdout().lock(),
                &mut password_store,
            ),
            "2" | "generate" | "g" => handle_generate_password(&mut stdout().lock()),
            "3" | "list" | "l" => handle_list_passwords(&mut stdout().lock(), &mut password_store),
            "4" | "remove" | "r" => handle_remove_password(
                &mut stdin().lock(),
                &mut stdout().lock(),
                &mut password_store,
            ),
            "5" | "show" | "s" => handle_show_password(
                &mut stdin().lock(),
                &mut stdout().lock(),
                &mut password_store,
            ),
            _ => break,
        }
    }
}

fn handle_add_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    password_store: &mut PasswordStore,
) {
    let message = [
        format!(
            "[{}] {} random password",
            "1".green().bold(),
            "generate".green().bold()
        ),
        format!(
            "[{}] {} your own password",
            "2".green().bold(),
            "enter".green().bold()
        ),
        format!("[{}] {}", "3".green().bold(), "cancel".green().bold()),
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
        password_store,
        service,
        username,
        None,
        generate,
        password_generator,
    ) {
        Ok(_) => writeln!(writer, "{}", "Password added successfully".green()).unwrap(),
        Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red()).unwrap(),
    };
}

fn handle_generate_password<W: Write>(writer: &mut W) {
    match generate_password(Length::Sixteen, false, true, true, true, 1, writer) {
        Ok(_) => (),
        Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red()).unwrap(),
    };
}

fn handle_list_passwords<W: Write>(writer: &mut W, password_store: &mut PasswordStore) {
    list_passwords(password_store, true, writer)
        .unwrap_or_else(|err| panic!("{}: {err}", "Failed to load passwords to store".red()));
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
    remove_password(writer, password_store, service, username)
        .unwrap_or_else(|err| panic!("{}: {err}", "Failed to remove password!".red()));
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
    if show_password(password_store, service, username, writer).is_err() {
        writeln!(writer, "Password not found").unwrap();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::NamedTempFile;

    // #[test]
    // fn test_repl() {
    //     let mut output = Vec::new();
    //     let mut mock_prompt_password = MockPromptPassword::new();
    //     mock_prompt_password
    //         .expect_prompt_password()
    //         .with(eq(format!(
    //             "Please enter the master password\n{}",
    //             ">> ".yellow()
    //         )))
    //         .times(1)
    //         .returning(|_| Ok("secret".to_string()));

    //     let temp_file = NamedTempFile::new().unwrap();
    //     let temp_file_name = temp_file.path().to_str().unwrap();
    //     repl(
    //         temp_file_name.to_string(),
    //         &mut output,
    //         &mock_prompt_password,
    //     );

    //     assert_eq!(
    //         String::from_utf8(output).unwrap(),
    //         format!("{}\n{}", "Welcome to L🦀CKBOX!".bold(), ">> ".yellow())
    //     );
    // }

    #[test]
    fn test_handle_add_password() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut password_store =
            PasswordStore::new(temp_file_name.to_string(), "secret".to_string()).unwrap();
        let mut input = b"1\ntest_service\ntest_username\n" as &[u8];
        let mut output = Vec::new();

        handle_add_password(&mut input, &mut output, &mut password_store);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(&format!(
            "[{}] {} random password",
            "1".green().bold(),
            "generate".green().bold()
        )));
        assert!(output_str.contains(&format!(
            "[{}] {} your own password",
            "2".green().bold(),
            "enter".green().bold()
        )));
        assert!(output_str.contains(&format!(
            "[{}] {}",
            "3".green().bold(),
            "cancel".green().bold()
        )));
        assert!(output_str.contains(&format!("{}Please enter the service name", ">> ".yellow())));
        assert!(output_str.contains(&format!(
            "{}Please enter the username (Optional)",
            ">> ".yellow()
        )));
        assert!(output_str.contains(&format!("{}", "Password added successfully".green())));
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
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut password_store =
            PasswordStore::new(temp_file_name.to_string(), "secret".to_string()).unwrap();
        add_password(
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
        assert!(output_str.contains(&format!(
            "Service: {}, Username: {}, Password: {}",
            "service".blue(),
            "username".blue(),
            "password".blue()
        )));
    }

    #[test]
    fn test_handle_remove_password() {
        let mut password_store =
            PasswordStore::new("test".to_string(), "secret".to_string()).unwrap();
        add_password(
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
        assert!(output_str.contains(&"Password not found".bright_yellow().to_string()));

        input = b"service\nusername\n" as &[u8];
        output = Vec::new();
        handle_remove_password(&mut input, &mut output, &mut password_store);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(&"Password deleted".green().to_string()));
    }

    #[test]
    fn test_handle_show_password() {
        let mut password_store =
            PasswordStore::new("test".to_string(), "secret".to_string()).unwrap();
        add_password(
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
}
