use crate::cli::Command;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use std::fmt::Display;

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
