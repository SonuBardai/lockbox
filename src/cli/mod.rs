use std::fmt::Display;

use clap::{builder::PossibleValue, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "lockbox", about = "A password manager and generator")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Parser)]
pub enum Length {
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl Length {
    pub fn get_val(self) -> usize {
        match self {
            Self::Eight => 8,
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = match self {
            Self::Eight => "8",
            Self::Sixteen => "16",
            Self::ThirtyTwo => "32",
        };
        write!(f, "{}", l)
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

#[derive(Parser, Debug)]
pub enum Command {
    Add {
        #[clap(short, long)]
        service: String,
        #[clap(short, long)]
        username: String,
        #[clap(short, long)]
        password: String,
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
    List,
    Remove {
        #[clap(short, long)]
        service: String,
        #[clap(short, long)]
        username: String,
    },
    Show {
        #[clap(short, long)]
        service: String,
        #[clap(short, long)]
        username: String,
    },
}

pub fn build_parser() -> Args {
    Args::parse()
}
