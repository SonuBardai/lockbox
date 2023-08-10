use clap::{builder::PossibleValue, Parser, ValueEnum};
use colored::Colorize;
use std::fmt::Display;
use terminal_size::{terminal_size, Height, Width};
const ASCII_ART_ABOUT: &str = r#"
            ..7J?..   ..^JJ7..
        :~~JPG5PB5PG ~#PPBBGGG5~~:
        ?&J?JPPY7:J& !@7~?##5??5&BJ
    :!PP5~JPB#57#GY ^PYJ7G#B77Y5GB!:
    7@BP555PPG#&Y^     ^JPBBP5555P@7
    :?##BGGGG7^^         ^~~5GGBBB?^
    7&P5P&?    .. .......    7@BPP&7
    .^5PGGG5~5PY5BB55GBG5G5~5PPBG5^.
        ~J!5BGJ77JGP??5GP7YPGBBBY~
^5BPPPPY5P5YY55J!7775Y77?5PGGPPPPBP~.
.^YG5JPGB#@#BBGP5555555555PGGB#@#BGGYY@G?.
7&Y:!?5&@&BB#@BBBPPPPPPPPPGB#&&B&@&5?75J5&:
7@^ !JPBBGGGGGBGGBBBBBBBBBB##GPGGBB5#P  !#:
7@^ GB5JP#:................... .BGJ5#P   .
:!. P#B7JP5!.                .~5PJ7B#Y
    7@.!GPGP?:            :7PGPG7.@?
    :~ :!PGG&~            !&BGG7: ~:
        ....              ....

"#;
pub const DEFAULT_PASSWORD_FILE_NAME: &str = "passwords";
const ABOUT: &str = "L🦀CKBOX: A password manager and generator";

fn get_about(terminal_size: Option<(Width, Height)>) -> String {
    if let Some((Width(w), Height(h))) = terminal_size {
        if w >= 45 && h >= 17 {
            let max_line_length = ASCII_ART_ABOUT
                .lines()
                .chain(ABOUT.lines())
                .map(|line| line.len())
                .max()
                .unwrap_or(0);
            let indent = (w as usize - max_line_length) / 2;
            let indented_ascii_art: String = ASCII_ART_ABOUT
                .lines()
                .map(|line| format!("{}{}", " ".repeat(indent), line))
                .collect::<Vec<String>>()
                .join("\n");
            let indented_about: String = ABOUT
                .bold()
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
            let max_line_length = ABOUT.lines().map(|line| line.len()).max().unwrap_or(0);
            let indent = (w as usize - max_line_length) / 2;
            let indented_about: String = ABOUT
                .bold()
                .lines()
                .map(|line| format!("{}{}", " ".repeat(indent), line))
                .collect::<Vec<String>>()
                .join("\n");
            indented_about.bold().to_string()
        }
    } else {
        ABOUT.to_string()
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
    Add {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILE_NAME.to_string())]
        file_name: String,
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
        #[clap(short, long, default_value_t = false)]
        generate: bool,
        #[clap(short, long, default_value_t = Length::Sixteen)]
        length: Length,
        #[clap(long, default_value_t = false)]
        symbols: bool,
        #[clap(long, default_value_t = true)]
        uppercase: bool,
        #[clap(long, default_value_t = true)]
        lowercase: bool,
        #[clap(long, default_value_t = true)]
        numbers: bool,
    },
    #[clap(
        about = "Generate a password with the specified properties [default: length=16, symbols=false, uppercase=true, lowercase=true, numbers=true, count=1]",
        long_about = "Generate a password with the specified properties [default: length=16, symbols=false, uppercase=true, lowercase=true, numbers=true, count=1]"
    )]
    Generate {
        #[clap(short, long, default_value_t = Length::Sixteen)]
        length: Length,
        #[clap(short, long, default_value_t = false)]
        symbols: bool,
        #[clap(short('U'), long, default_value_t = true)]
        uppercase: bool,
        #[clap(short('u'), long, default_value_t = true)]
        lowercase: bool,
        #[clap(short, long, default_value_t = true)]
        numbers: bool,
        #[clap(short, long, default_value_t = 1)]
        count: usize,
    },
    List {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILE_NAME.to_string())]
        file_name: String,
        #[clap(short, long)]
        master: Option<String>,
        #[clap(short, long, default_value_t = false, aliases=&["show", "show-passwords", "reveal"])]
        show_passwords: bool,
    },
    Remove {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILE_NAME.to_string())]
        file_name: String,
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
    Show {
        #[clap(short, long, default_value_t=DEFAULT_PASSWORD_FILE_NAME.to_string())]
        file_name: String,
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
    Repl,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case(Some((Width(80), Height(10))), format!("{}{}", ASCII_ART_ABOUT, ABOUT)),
        case(Some((Width(79), Height(10))), ABOUT.to_string()),
        case(None, ABOUT.to_string())
    )]
    fn test_get_about(input: Option<(Width, Height)>, expected: String) {
        assert_eq!(get_about(input), expected);
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
                file_name: DEFAULT_PASSWORD_FILE_NAME.to_string(),
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
                file_name: DEFAULT_PASSWORD_FILE_NAME.to_string(),
                master: Some("master_password".to_string()),
                show_passwords: false,
            },
        }
    ),
    case(
        &["lockbox", "remove", "-s", "service"],
        Args {
            command: Command::Remove {
                file_name: DEFAULT_PASSWORD_FILE_NAME.to_string(),
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
                file_name: DEFAULT_PASSWORD_FILE_NAME.to_string(),
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
