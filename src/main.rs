use lockbox::cli::{build_parser, Command};
use passwords::PasswordGenerator;

fn main() {
    let args = build_parser();
    match args.command {
        Command::Add {
            service,
            username,
            password,
        } => {
            println!("Add operation.");
            println!(
                "Service: {}, Username: {}, Password: {}",
                service, username, password
            );
        }
        Command::Generate {
            length,
            symbols,
            uppercase,
            lowercase,
            numbers,
            count,
        } => {
            let pg = PasswordGenerator::new()
                .length(length.get_val())
                .lowercase_letters(lowercase)
                .uppercase_letters(uppercase)
                .numbers(numbers)
                .symbols(symbols)
                .strict(true);
            if count > 1 {
                match pg.generate(count) {
                    Ok(passwords) => {
                        for password in passwords {
                            println!("{}", password)
                        }
                    }
                    Err(err) => println!("Error generating password: {}", err),
                }
            } else {
                match pg.generate_one() {
                    Ok(password) => println!("{}", password),
                    Err(err) => println!("Error generating password: {}", err),
                }
            }
        }
        Command::List => {
            println!("List operation.");
        }
        Command::Remove { service, username } => {
            println!("Add operation.");
            println!("Service: {:?}, Username: {}", service, username);
        }
        Command::Show { service, username } => {
            println!("Show operation.");
            println!("Service: {:?}, Username: {}", service, username);
        }
    }
}
