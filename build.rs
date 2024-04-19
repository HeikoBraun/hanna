use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs;
use std::io::Error;

include!("src/cli.rs");

const BINNAME: &str = "hannes";

fn main() -> Result<(), Error> {
    let out_dir = "./completions";
    fs::create_dir_all(out_dir)?;

    let mut cmd = Cli::command();

    for &shell in [&Shell::Zsh, &Shell::Bash] {
        let path = generate_to(shell, &mut cmd, BINNAME, out_dir)?;
        println!("cargo:info=Completion file for {shell} was generated: {path:?}");
    }

    Ok(())
}
