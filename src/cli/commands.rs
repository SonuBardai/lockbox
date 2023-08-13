use crate::{
    cli::{args::Length, io::read_hidden_input},
    store::PasswordStore,
};
use anyhow::anyhow;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use passwords::PasswordGenerator;
use std::io::Write;

pub fn copy_to_clipboard(password: String) -> anyhow::Result<()> {
    let ctx_result: Result<ClipboardContext, _> = ClipboardProvider::new();
    let mut ctx = ctx_result.map_err(|_| anyhow!("Unable to initialize clipboard"))?;
    ctx.set_contents(password)
        .map_err(|_| anyhow!("Unable to set clipboard contents"))?;
    Ok(())
}

pub fn add_password(
    password_store: &mut PasswordStore,
    service: String,
    username: Option<String>,
    password: Option<String>,
    generate: bool,
    password_generator: PasswordGenerator,
) -> anyhow::Result<()> {
    let password = if generate {
        let password = password_generator
            .generate_one()
            .unwrap_or_else(|_| panic!("{}", "Failed to generate password".red()));
        match copy_to_clipboard(password.clone()) {
            Ok(_) => println!("Random password generated and copied to clipboard"),
            Err(err) => {
                println!("Random password generated");
                println!("Note: Failed to copy password to clipboard: {}", err);
            }
        }
        password
    } else {
        password.unwrap_or_else(|| read_hidden_input("password"))
    };
    password_store
        .load()?
        .push(service, username, password)?
        .dump()?;
    Ok(())
}

pub fn generate_password(
    length: Length,
    symbols: bool,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    count: usize,
    writer: &mut dyn Write,
) -> anyhow::Result<()> {
    let password_generator = PasswordGenerator::new()
        .length(length.get_val())
        .lowercase_letters(lowercase)
        .uppercase_letters(uppercase)
        .numbers(numbers)
        .symbols(symbols)
        .strict(true);
    writeln!(writer)?;
    if count > 1 {
        match password_generator.generate(count) {
            Ok(passwords) => {
                for password in passwords {
                    writeln!(writer, "{}", password.green())?
                }
            }
            Err(err) => writeln!(
                writer,
                "{}",
                format!("Error generating password: {}", err).red()
            )?,
        }
    } else {
        match password_generator.generate_one() {
            Ok(password) => {
                match copy_to_clipboard(password.clone()) {
                    Ok(_) => (),
                    Err(err) => {
                        writeln!(writer, "Random password generated")?;
                        writeln!(
                            writer,
                            "{}",
                            format!("Note: Failed to copy password to clipboard: {}", err).yellow()
                        )?;
                    }
                }
                if copy_to_clipboard(password.clone()).is_ok() {
                    writeln!(writer, "{} (Copied to Clipboard)", password.green())?;
                } else {
                    writeln!(writer, "{}", password.green())?;
                    writeln!(
                        writer,
                        "{}",
                        "Note: Failed to copy password to clipboard".red()
                    )?
                }
            }
            Err(err) => writeln!(
                writer,
                "{}",
                format!("Error generating password: {}", err).red()
            )?,
        }
    }
    Ok(())
}

pub fn show_password(
    password_store: &mut PasswordStore,
    service: String,
    username: Option<String>,
    writer: &mut dyn Write,
) -> anyhow::Result<()> {
    let password = password_store.load()?.find(service, username);
    if let Some(password) = password {
        password.print_password(Some(Color::Blue), writer)?;
    } else {
        writeln!(writer, "Password not found")?;
    }
    Ok(())
}

pub fn list_passwords(
    password_store: &mut PasswordStore,
    show_passwords: bool,
) -> anyhow::Result<()> {
    password_store
        .load()?
        .print(show_passwords, Some(Color::Blue));
    Ok(())
}

pub fn remove_password(
    password_store: &mut PasswordStore,
    service: String,
    username: Option<String>,
) -> anyhow::Result<()> {
    password_store.load()?.pop(service, username).dump()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use passwords::PasswordGenerator;
    use rstest::rstest;
    use tempfile::NamedTempFile;

    #[rstest]
    #[case("service1".to_string(), Some("username1".to_string()), Some("password1"), false)]
    #[case("service2".to_string(), None, Some("password2"), false)]
    #[case("service3".to_string(), Some("username3".to_string()), None, true)]
    fn test_add_password(
        #[case] service: String,
        #[case] username: Option<String>,
        #[case] password: Option<&str>,
        #[case] generate: bool,
    ) {
        let password_generator = PasswordGenerator::new()
            .length(10)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .numbers(true)
            .symbols(true)
            .strict(true);
        let master = "master_password".to_string();
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut password_store = PasswordStore::new(temp_file_name.to_string(), master).unwrap();
        let result = add_password(
            &mut password_store,
            service.clone(),
            username.clone(),
            password.map(|s| s.to_string()),
            generate,
            password_generator,
        );
        assert!(result.is_ok());
        assert!(password_store.find(service, username).is_some());
    }

    #[rstest]
    #[case(Length::Eight, true, true, true, true, 1)]
    #[case(Length::Sixteen, false, true, true, true, 2)]
    #[case(Length::ThirtyTwo, true, false, true, true, 3)]
    #[case(Length::Sixteen, true, false, false, false, 2)]
    #[case(Length::ThirtyTwo, false, true, false, false, 3)]
    fn test_generate_password(
        #[case] length: Length,
        #[case] symbols: bool,
        #[case] uppercase: bool,
        #[case] lowercase: bool,
        #[case] numbers: bool,
        #[case] count: usize,
    ) {
        let mut output = Vec::new();
        let mut writer = std::io::Cursor::new(output);
        generate_password(
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
            &mut writer,
        )
        .unwrap();
        output = writer.into_inner();
        let output_str = String::from_utf8(output).unwrap();
        println!("{}", output_str);
        let lines: Vec<&str> = output_str.trim().lines().collect();
        assert_eq!(lines.len(), count);
        if count == 1 {
            println!("{}", output_str);
            assert!(output_str.contains("(Copied to Clipboard)"))
        }
    }

    #[test]
    fn test_generate_password_all_false() {
        let mut output = Vec::new();
        let mut writer = std::io::Cursor::new(output);
        generate_password(Length::Eight, false, false, false, false, 1, &mut writer).unwrap();
        output = writer.into_inner();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(
            "Error generating password: You need to enable at least one kind of characters."
        ))
    }

    #[rstest]
    #[case("service1".to_string(), Some("username1".to_string()), "password1".to_string())]
    #[case("service2", None, "password2".to_string())]
    fn test_show_password(
        #[case] service: String,
        #[case] username: Option<String>,
        #[case] password: String,
    ) {
        let master = "master_password".to_string();
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_name = temp_file.path().to_str().unwrap();
        let mut password_store = PasswordStore::new(temp_file_name.to_string(), master).unwrap();
        add_password(
            &mut password_store,
            service.clone(),
            username.clone(),
            Some(password.clone()),
            false,
            PasswordGenerator::default(),
        )
        .unwrap();

        let mut output = Vec::new();
        let mut writer = std::io::Cursor::new(output);
        let result = show_password(&mut password_store, service, username, &mut writer);
        assert!(result.is_ok());

        output = writer.into_inner();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(&password));
    }
}
