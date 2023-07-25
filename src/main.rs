use clap::Parser;
use lockbox::{cli::args::Args, cli::Command};

fn main() {
    Command::map(Args::parse())
}
