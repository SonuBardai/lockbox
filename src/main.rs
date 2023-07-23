use clap::{builder::PossibleValue, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "lockbox", about = "A password manager and generator")]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Length {
    Eight,
    Sixteen,
    ThirtyTwo,
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
enum Command {
    Add {
        #[clap(short, long)]
        service: String,
        #[clap(short, long)]
        username: String,
        #[clap(short, long)]
        password: String,
    },
    Generate {
        #[clap(short, long)]
        length: Length,
        #[clap(short, long)]
        symbols: bool,
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

fn main() {
    let args = Args::parse();
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
        Command::Generate { length, symbols } => {
            println!("Generate operation.");
            println!("Length: {:?}, Symbols: {}", length, symbols);
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
