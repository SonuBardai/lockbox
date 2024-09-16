use clap::Parser;
use lock_box::{
    cli::{
        args::{Args, DEFAULT_PASSWORD_FILENAME},
        io::RpasswordPromptPassword,
        run_cli,
    },
    config::LockboxConfig,
    repl::repl,
};

fn main() {
    let config = LockboxConfig::read().unwrap_or_else(|err| {
        println!("Failed to read config: {err}");
    });
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    let prompt_password = &RpasswordPromptPassword;
    if std::env::args().len() == 1 {
        repl(&mut input, &mut output, prompt_password, config)
    } else {
        let args = Args::parse();
        run_cli(&mut input, &mut output, prompt_password, args, config);
    }
}
