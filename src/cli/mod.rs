pub mod args;
pub mod commands;
pub mod io;

use self::{
    args::{get_password_store_path, Args, Command, DEFAULT_PASSWORD_FILENAME},
    commands::{
        add_password, generate_password, list_passwords, remove_password, show_password,
        update_master_password,
    },
    io::{read_hidden_input, PromptPassword},
};
use crate::{repl::repl, store::PasswordStore};
use colored::*;
use passwords::PasswordGenerator;
use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

pub fn run_cli<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt_password: &dyn PromptPassword,
    args: Args,
) {
    match args.command {
        Command::Add {
            file_name,
            service,
            username,
            password,
            master,
            generate,
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
        } => {
            let password_generator = PasswordGenerator::new()
                .length(length.get_val())
                .lowercase_letters(lowercase)
                .uppercase_letters(uppercase)
                .numbers(numbers)
                .symbols(symbols)
                .strict(true);
            let master =
                master.unwrap_or_else(|| read_hidden_input("master password", prompt_password));
            let file_path = get_password_store_path(file_name)
                .unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
            let mut password_store = match PasswordStore::new(file_path, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            match add_password(
                writer,
                prompt_password,
                &mut password_store,
                service,
                username,
                password,
                generate,
                password_generator,
            ) {
                Ok(_) => writeln!(writer, "{}", "Password added successfully".green()).unwrap(),
                Err(err) => {
                    writeln!(writer, "{}", format!("Error: {}", err).red()).unwrap();
                }
            }
        }
        Command::Generate {
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
        } => match generate_password(
            length, symbols, uppercase, lowercase, numbers, count, writer,
        ) {
            Ok(_) => (),
            Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red())
                .unwrap_or_else(|_| println!("{}", format!("Error: {}", err).red())),
        },
        Command::List {
            file_name,
            master,
            show_passwords,
        } => {
            let master =
                master.unwrap_or_else(|| read_hidden_input("master password", prompt_password));
            let file_path = get_password_store_path(file_name)
                .unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
            let mut password_store = match PasswordStore::new(file_path, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            match list_passwords(&mut password_store, show_passwords, writer) {
                Ok(_) => (),
                Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red())
                    .unwrap_or_else(|_| println!("{}", format!("Error: {}", err).red())),
            }
        }
        Command::Remove {
            file_name,
            service,
            username,
            master,
        } => {
            let master =
                master.unwrap_or_else(|| read_hidden_input("master password", prompt_password));
            let file_path = get_password_store_path(file_name)
                .unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
            let mut password_store = match PasswordStore::new(file_path, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            match remove_password(writer, &mut password_store, service, username) {
                Ok(_) => writeln!(writer, "Password removed successfully")
                    .unwrap_or_else(|_| println!("Password removed successfully")),
                Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red())
                    .unwrap_or_else(|_| println!("{}", format!("Error: {}", err).red())),
            }
        }
        Command::Show {
            file_name,
            service,
            username,
            master,
        } => {
            let master =
                master.unwrap_or_else(|| read_hidden_input("master password", prompt_password));
            let file_path = get_password_store_path(file_name)
                .unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
            let mut password_store = match PasswordStore::new(file_path, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            match show_password(&mut password_store, service, username, writer) {
                Ok(_) => (),
                Err(err) => writeln!(writer, "{}", format!("Error: {}", err).red())
                    .unwrap_or_else(|_| println!("{}", format!("Error: {}", err).red())),
            }
        }
        Command::UpdateMaster {
            file_name,
            master,
            new_master,
        } => {
            let master =
                master.unwrap_or_else(|| read_hidden_input("master password", prompt_password));
            let new_master = new_master
                .unwrap_or_else(|| read_hidden_input("new master password", prompt_password));
            let file_path = get_password_store_path(file_name)
                .unwrap_or(PathBuf::from(DEFAULT_PASSWORD_FILENAME));
            let mut password_store = match PasswordStore::new(file_path, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            update_master_password(writer, new_master, &mut password_store).unwrap_or_else(|err| {
                writeln!(
                    writer,
                    "{}: {err}",
                    "Failed to update master password".red()
                )
                .unwrap_or_else(|_| {
                    println!("{}: {err}", "Failed to update master password".red())
                });
            });
        }
        Command::Repl { file_name } => repl(reader, writer, prompt_password, file_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::io::MockPromptPassword;
    use clap::Parser;
    use rstest::rstest;
    use std::io::Cursor;

    use tempfile::NamedTempFile;

    #[rstest(
        args,
        input,
        expected_output,
        use_temp_file,
        case(
            vec!["lockbox", "add", "--service", "test_service", "--generate", "--master", "test_master_password"],
            b"",
            &"Password added successfully".green().to_string(),
            true
        ),
        case(
            vec!["lockbox", "generate"],
            b"",
            "Random password generated.",
            false
        ),
        case(
            vec!["lockbox", "list", "--master", "test_master_password", "--reveal"],
            b"",
            &format!("Service: {}, Username: {}, Password: {}", "service".blue(), "username".blue(), "password".blue()),
            true
        ),
        case(
            vec!["lockbox", "remove", "--service", "service", "--username", "username", "--master", "test_master_password"],
            b"",
            "Password removed successfully\n",
            true
        ),
        case(
            vec!["lockbox", "show", "--service", "service", "--username", "username", "--master", "test_master_password"],
            b"",
            &format!("Password: {}", "password".blue()),
            true
        ),
        case(
            vec!["lockbox", "update-master", "--master", "test_master_password", "--new-master", "new_master_password"],
            b"",
            &"Master password updated successfully".green().to_string(),
            true
        )

    )]
    fn test_run_cli(args: Vec<&str>, input: &[u8], expected_output: &str, use_temp_file: bool) {
        let mut args = args;
        let temp_file = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut temp_writer = std::io::Cursor::new(Vec::new());

        let mut password_store =
            PasswordStore::new(temp_file.clone(), "test_master_password".to_string()).unwrap();
        let mock_prompt_password = &MockPromptPassword::new();
        add_password(
            &mut temp_writer,
            mock_prompt_password,
            &mut password_store,
            "service".to_string(),
            Some("username".to_string()),
            Some("password".to_string()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();

        let temp_file_str = temp_file.to_string_lossy().to_string();
        if use_temp_file {
            args.push("--file-name");
            args.push(&temp_file_str);
        }
        let args = Args::parse_from(args);

        let mut input = Cursor::new(input);
        let mut output = Vec::new();
        let mock_prompt_password = &MockPromptPassword::new();

        run_cli(&mut input, &mut output, mock_prompt_password, args);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(expected_output));
    }

    #[test]
    fn test_run_cli_repl() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let args = Args::parse_from(vec!["lockbox", "repl", "--file-name", temp_file_name]);
        let mut input = b"exit\n" as &[u8];
        let mut output = Vec::new();
        let mut mock_prompt_password = MockPromptPassword::new();
        mock_prompt_password
            .expect_prompt_password()
            .returning(|_| Ok("password\n".to_string()));
        run_cli(&mut input, &mut output, &mock_prompt_password, args);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(&"Welcome to L🦀CKBOX!\n".bold().to_string()));
        assert!(output_str.contains(
            &[
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
                format!(
                    "[{}] {} password",
                    "6".green().bold(),
                    "update master".green().bold()
                ),
                format!("[{}] {}", "7".green().bold(), "exit".green().bold()),
            ]
            .join(" ")
        ));
    }
}
