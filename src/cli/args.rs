use clap::{builder::PossibleValue, Parser, ValueEnum};
use std::fmt::Display;

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "lockbox", about = "A password manager and generator")]
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
        #[clap(short, long)]
        master: Option<String>,
        #[clap(short, long, default_value_t = false, aliases=&["show", "show-passwords", "reveal"])]
        show_passwords: bool,
    },
    Remove {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
    Show {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    input,
    expected,
    case(
        &["lockbox", "add", "-s", "test_service", "-u", "test_username", "-p", "test_password"],
        Args {
            command: Command::Add {
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
                master: Some("master_password".to_string()),
                show_passwords: false,
            },
        }
    ),
    case(
        &["lockbox", "remove", "-s", "service"],
        Args {
            command: Command::Remove {
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
}
