use clap::Parser;
use lock_box::{
    cli::{
        args::{Args, DEFAULT_PASSWORD_FILENAME},
        io::RpasswordPromptPassword,
        run_cli,
    },
    repl::repl,
};

fn main() {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    let prompt_password = &RpasswordPromptPassword;
    if std::env::args().len() == 1 {
        repl(&mut input,&mut output,prompt_password,DEFAULT_PASSWORD_FILENAME.to_string())
    } else {
        let args = Args::parse();
        run_cli(&mut input, &mut output, prompt_password, args);
    }
}
