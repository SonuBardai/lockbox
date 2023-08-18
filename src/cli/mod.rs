pub mod args;
pub mod commands;
pub mod io;

use self::{
    args::{Args, Command, DEFAULT_PASSWORD_FILE_NAME},
    commands::{
        add_password, generate_password, list_passwords, remove_password, show_password,
        update_master_password,
    },
    io::{read_hidden_input, PromptPassword},
};
use crate::{repl::repl, store::PasswordStore};
use colored::*;
use passwords::PasswordGenerator;
use std::io::{BufRead, Write};

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
            let mut password_store = match PasswordStore::new(file_name, master) {
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
            let mut password_store = match PasswordStore::new(file_name, master) {
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
            let mut password_store = match PasswordStore::new(file_name, master) {
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
            let mut password_store = match PasswordStore::new(file_name, master) {
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
            let mut password_store = match PasswordStore::new(file_name, master) {
                Ok(password_store) => password_store,
                Err(err) => {
                    writeln!(writer, "{}", err).unwrap_or_else(|_| println!("{}", err));
                    return;
                }
            };
            update_master_password(new_master, &mut password_store).unwrap_or_else(|err| {
                writeln!(
                    writer,
                    "{}: {err}",
                    "Failed to update master password".red()
                )
                .unwrap_or_else(|_| println!("{}: {err}", "Failed to update master password".red()))
            });
        }
        Command::Repl => repl(
            reader,
            writer,
            prompt_password,
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
        ),
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
        )
    )]
    fn test_run_cli(args: Vec<&str>, input: &[u8], expected_output: &str, use_temp_file: bool) {
        let mut args = args;
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut temp_writer = std::io::Cursor::new(Vec::new());

        let mut password_store = PasswordStore::new(
            temp_file_name.to_string(),
            "test_master_password".to_string(),
        )
        .unwrap();
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

        if use_temp_file {
            args.push("--file-name");
            args.push(temp_file_name);
        }
        let args = Args::parse_from(args);

        let mut input = Cursor::new(input);
        let mut output = Vec::new();
        let mock_prompt_password = &MockPromptPassword::new();

        run_cli(&mut input, &mut output, mock_prompt_password, args);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(expected_output));
    }
}
