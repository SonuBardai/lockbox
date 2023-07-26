pub mod args;
pub mod commands;
use crate::cli::{
    args::{Args, Length},
    commands::{add_password, generate_password, list_passwords, remove_password, show_password},
};
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    Add {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        password: String,
        #[clap(short, long)]
        master: String,
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
        master: String,
    },
    Remove {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: String,
    },
    Show {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: String,
    },
}

impl Command {
    pub fn map(cli_args: Args) {
        match cli_args.command {
            Command::Add {
                service,
                username,
                password,
                master,
            } => add_password(service, username, master, password).expect("Failed to add password"),
            Command::Generate {
                length,
                symbols,
                uppercase,
                lowercase,
                numbers,
                count,
            } => generate_password(length, symbols, uppercase, lowercase, numbers, count),
            Command::List { master } => list_passwords(master).expect("Failed to get passwords"),
            Command::Remove {
                service,
                username,
                master,
            } => remove_password(service, username, master).expect("Failed to remove password"),
            Command::Show {
                service,
                username,
                master,
            } => show_password(service, username, master).expect("Failed to get passwords"),
        }
    }
}
