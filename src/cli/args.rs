use anyhow::Ok;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use colored::Colorize;
use std::{env, fs::create_dir_all};
use std::{fmt::Display, path::PathBuf};
use terminal_size::{terminal_size, Height, Width};
const ASCII_ART_ABOUT: &str = r#"
            ..7J?..   ..^JJ7..
        :~~JPG5PB5PG ~#PPBBGGG5~~:
        ?&J?JPPY7:J& !@7~?##5??5&BJ
    :!PP5~JPB#57#GY ^PYJ7G#B77Y5GB!:
    7@BP555PPG#&Y^     ^JPBBP5555P@7
    :?##BGGGG7^^         ^~~5GGBBB?^
    7&P5P&?    .. .......    7@BPP&7
    .^5PGGG5~OCKBOXLOCKBOXL~5PPBG5^.
        ~LOCKBOXLOCKBOXLOCKBOXLOC~
^CKBOXLOCKBOXLOCKBOXLOCKBOXLOCKBOXL~.
.^CKBOXLOCKBOXLOCKBOXLOCKBOXLOCKBOXLOCKG?.
7&Y:!CKBOXLOCKBOXLOCKBOXLOCKBOXLOCKBOXLOCK:
7@^ !LOCKBOXLOCKBOXLOCKBOXLOCKBOXLOCKB  OX:
7@^ GB5JP#:................... .BGJ5#P   .
:!. P#B7JP5!.                .~5PJ7B#Y
    7@.!GPGP?:            :7PGPG7.@?
    :~ :!PGG&~            !&BGG7: ~:
        ....              ....

"#;
const ABOUT: &str = "L🦀CKBOX: A password manager and generator";
pub const DEFAULT_PASSWORD_FILENAME: &str = "store";

pub fn get_default_password_filename(file_name: String) -> anyhow::Result<PathBuf> {
    #[cfg(not(windows))]
    let home_dir = env::var("HOME")?;
    #[cfg(windows)]
    let home_dir = env::var("USERPROFILE")?;
    let home_path = PathBuf::from(home_dir);
    let file_path = home_path.join(".lockbox").join(file_name);
    create_dir_all(file_path.parent().unwrap())?;
    Ok(file_path)
}

fn get_about(terminal_size: Option<(Width, Height)>) -> String {
    let about = ABOUT.bold();
    if let Some((Width(w), Height(h))) = terminal_size {
        let ascii_art_lines = ASCII_ART_ABOUT.lines().collect::<Vec<&str>>();
        let ascii_art_height = ascii_art_lines.len();
        let ascii_art_width = ascii_art_lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);

        let max_line_length = if w as usize >= ascii_art_width && h as usize >= ascii_art_height {
            ASCII_ART_ABOUT
                .lines()
                .chain(about.lines())
                .map(|line| line.len())
                .max()
                .unwrap_or(0)
        } else {
            about.lines().map(|line| line.len()).max().unwrap_or(0)
        };
        let indent = if max_line_length > w as usize {
            0
        } else {
            (w as usize - max_line_length) / 2
        };
        let indented_about: String = about
            .lines()
            .map(|line| format!("{}{}", " ".repeat(indent), line))
            .collect::<Vec<String>>()
            .join("\n");
        if w >= 45 && h >= 17 {
            let indented_ascii_art: String = ASCII_ART_ABOUT
                .lines()
                .map(|line| format!("{}{}", " ".repeat(indent), line))
                .collect::<Vec<String>>()
                .join("\n");
            format!(
                "{}\n{}",
                indented_ascii_art.bright_red(),
                indented_about.bold()
            )
        } else {
            indented_about.bold().to_string()
        }
    } else {
        about.to_string()
    }
}

#[derive(Parser, Debug, PartialEq)]
#[clap(
    name = "lockbox",
    about = get_about(terminal_size()),
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Copy, Clone, Parser, PartialEq)]
pub enum Length {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}

impl Length {
    pub fn get_val(self) -> usize {
        self as usize
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

impl ValueEnum for Length {
    fn value_variants<'a>() -> &'a [Self] {
        &[Length::Eight, Length::Sixteen, Length::ThirtyTwo]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Length::Eight => Some(PossibleValue::new("8")),
            Length::Sixteen => Some(PossibleValue::new("16")),
            Length::ThirtyTwo => Some(PossibleValue::new("32")),
        }
    }
}

#[derive(Parser, Debug, PartialEq)]
pub enum Command {
    #[clap(
        about = "Add a new password to the password manager",
        long_about = "Use this command to add a new password entry to your password store. You can specify the service, username, and password, or choose to generate a new password with custom properties. You can also specify the name of the password file and the master password used to encrypt the password store."
    )]
    Add {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use.")]
        file_name: String,
        #[clap(
            short,
            long,
            help = "The name of the service for which you are adding a password. [default: passwords]"
        )]
        service: String,
        #[clap(short, long, aliases=&["user"], help="The username associated with the password. [Optional]")]
        username: Option<String>,
        #[clap(short, long, help = "The password to add.")]
        password: Option<String>,
        #[clap(
            short,
            long,
            help = "The master password used to encrypt the password store."
        )]
        master: Option<String>,
        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Whether to generate a new password instead of specifying one. [default: false]"
        )]
        generate: bool,
        #[clap(short, long, default_value_t = Length::Sixteen, help="The length of the generated password.")]
        length: Length,
        #[clap(
            long,
            default_value_t = false,
            help = "Whether to include symbols in the generated password. [default: false]"
        )]
        symbols: bool,
        #[clap(
            long,
            default_value_t = true,
            help = "Whether to include uppercase letters in the generated password. [default: true]"
        )]
        uppercase: bool,
        #[clap(
            long,
            default_value_t = true,
            help = "Whether to include lowercase letters in the generated password. [default: true]"
        )]
        lowercase: bool,
        #[clap(
            long,
            default_value_t = true,
            help = "Whether to include numbers in the generated password. [default: true]"
        )]
        numbers: bool,
    },

    #[clap(
        about = "Generate a random password.",
        long_about = "Use this command to generate a random password with custom properties. You can specify the length of the generated password and choose whether to include symbols, uppercase letters, lowercase letters, and numbers. You can also generate multiple passwords at once by specifying the count option."
    )]
    Generate {
        #[clap(short, long, default_value_t = Length::Sixteen, help = "The length of the generated password.")]
        length: Length,
        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Whether to include symbols in the generated password. [default: false]"
        )]
        symbols: bool,
        #[clap(
            short('U'),
            long,
            default_value_t = true,
            help = "Whether to include uppercase letters in the generated password. [default: true]"
        )]
        uppercase: bool,
        #[clap(
            short('u'),
            long,
            default_value_t = true,
            help = "Whether to include lowercase letters in the generated password. [default: true]"
        )]
        lowercase: bool,
        #[clap(
            short,
            long,
            default_value_t = true,
            help = "Whether to include numbers in the generated password. [default: true]"
        )]
        numbers: bool,
        #[clap(
            short,
            long,
            default_value_t = 1,
            help = "The number of passwords to generate. [default: 1]"
        )]
        count: usize,
    },

    #[clap(
        about = "List all passwords in the password manager",
        long_about = "Use this command to list all passwords stored in your password manager. You can specify the name of the password file and the master password used to decrypt the password store. You can also choose whether to show the actual passwords or just the service and username information."
    )]
    List {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use. [default: passwords]")]
        file_name: String,
        #[clap(
            short,
            long,
            help = "The master password used to decrypt the password store"
        )]
        master: Option<String>,
        #[clap(short, long, default_value_t = false, aliases=&["show", "show-passwords", "reveal"], help="Whether to show the actual passwords or just the service and username information. [default: false]")]
        show_passwords: bool,
    },

    #[clap(
        about = "Remove a password from the password manager",
        long_about = "Use this command to remove a password entry from your password store. You can specify the service and username associated with the password you want to remove. You can also specify the name of the password file and the master password used to encrypt the password store."
    )]
    Remove {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use. [default: passwords]")]
        file_name: String,
        #[clap(
            short,
            long,
            help = "The name of the service for which you are removing a password."
        )]
        service: String,
        #[clap(short, long, aliases=&["user"], help="The username associated with the password you want to remove. [Optional]")]
        username: Option<String>,
        #[clap(
            short,
            long,
            help = "The master password used to encrypt the password store."
        )]
        master: Option<String>,
    },

    #[clap(
        about = "Show a specific password in the password manager",
        long_about = "Use this command to show a specific password stored in your password manager. You can specify the service and username associated with the password you want to show. You can also specify the name of the password file and the master password used to decrypt the password store."
    )]
    Show {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use. [default: passwords]")]
        file_name: String,
        #[clap(
            short,
            long,
            help = "The name of the service for which you are showing a password."
        )]
        service: String,
        #[clap(short, long, aliases=&["user"], help="The username associated with the password you want to show. [Optional]")]
        username: Option<String>,
        #[clap(
            short,
            long,
            help = "The master password used to decrypt the password store."
        )]
        master: Option<String>,
    },

    #[clap(
        about = "Update the master password",
        long_about = "Update the master password used to encrypt and decrypt the password store"
    )]
    UpdateMaster {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use. [default: passwords]")]
        file_name: String,
        #[clap(
            short,
            long,
            help = "The original master password used to encrypt and decrypt the password store."
        )]
        master: Option<String>,
        #[clap(
            short,
            long,
            help = "The new master password to be used to encrypt and decrypt the password store."
        )]
        new_master: Option<String>,
    },

    #[clap(
        about = "Start an interactive REPL session",
        long_about = "Use this command to start an interactive REPL (Read-Eval-Print Loop) session with your password manager. In this mode, you can enter commands interactively and see their results immediately."
    )]
    Repl {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILENAME.to_string(), help="The name of the password file to use. [default: passwords]")]
        file_name: String,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        case(Some((Width(10), Height(10)))),
        case(Some((Width(1000), Height(10)))),
        case(Some((Width(10), Height(1000)))),
        case(Some((Width(50), Height(20)))),
        case(Some((Width(500), Height(200)))),
        case(Some((Width(70), Height(40)))),
        case(None),
    )]
    fn test_get_about(input: Option<(Width, Height)>) {
        let received = get_about(input);
        assert!(received.contains(ABOUT));
        let ascii_art_lines = ASCII_ART_ABOUT.lines().collect::<Vec<&str>>();
        let ascii_art_height = ascii_art_lines.len();
        let ascii_art_width = ascii_art_lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);
        if let Some((Width(w), Height(h))) = input {
            if w as usize >= ascii_art_width && h as usize >= ascii_art_height {
                assert!(received.contains(ASCII_ART_ABOUT.lines().next().unwrap()))
            }
        }
    }

    #[rstest(
    input,
    expected,
    case(
        &["lockbox", "add", "-f", "test_passwords", "-s", "test_service", "-u", "test_username", "-p", "test_password"],
        Args {
            command: Command::Add {
                file_name: "test_passwords".to_string(),
                service: "test_service".to_string(),
                username: Some("test_username".to_string()),
                password: Some("test_password".to_string()),
                master: None,
                generate: false,
                length: Length::Sixteen,
                symbols: false,
                uppercase: true,
                lowercase: true,
                numbers: true,
            },
        }
    ),
    case(
        &["lockbox", "add", "-s", "test_service", "-u", "test_username", "-p", "test_password"],
        Args {
            command: Command::Add {
                file_name: DEFAULT_PASSWORD_FILENAME.to_string(),
                service: "test_service".to_string(),
                username: Some("test_username".to_string()),
                password: Some("test_password".to_string()),
                master: None,
                generate: false,
                length: Length::Sixteen,
                symbols: false,
                uppercase: true,
                lowercase: true,
                numbers: true,
            },
        }
    ),
    case(
        &["lockbox", "generate", "-l", "32", "-s"],
        Args {
            command: Command::Generate {
                length: Length::ThirtyTwo,
                symbols: true,
                uppercase: true,
                lowercase: true,
                numbers: true,
                count: 1,
            },
        }
    ),
    case(
        &["lockbox", "list", "--master", "master_password"],
        Args {
            command: Command::List {
                file_name: DEFAULT_PASSWORD_FILENAME.to_string(),
                master: Some("master_password".to_string()),
                show_passwords: false,
            },
        }
    ),
    case(
        &["lockbox", "remove", "-s", "service"],
        Args {
            command: Command::Remove {
                file_name: DEFAULT_PASSWORD_FILENAME.to_string(),
                service: "service".to_string(),
                username: None,
                master: None,
            },
        }
    ),
    case(
        &["lockbox", "show", "-s", "service"],
        Args {
            command: Command::Show {
                file_name: DEFAULT_PASSWORD_FILENAME.to_string(),
                service: "service".to_string(),
                username: None,
                master: None,
            },
        }
    )
    )]
    fn test_args(input: &[&str], expected: Args) {
        let args = Args::parse_from(input);
        assert_eq!(args, expected);
    }

    #[rstest(
        input,
        expected,
        case(Length::Eight, 8),
        case(Length::Sixteen, 16),
        case(Length::ThirtyTwo, 32)
    )]
    fn test_length_get_val(input: Length, expected: usize) {
        assert_eq!(input.get_val(), expected)
    }
}
