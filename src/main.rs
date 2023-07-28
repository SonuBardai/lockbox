use clap::Parser;
use lockbox::{cli::args::Args, cli::commands::Command};

fn main() {
    Command::map(Args::parse())
}
