pub mod cli;
pub mod cmds;
pub mod error;
use clap::Parser;
use console::style;
use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::Cli::parse().run() {
        Ok(out) => {
            println!("{}", style(out).for_stdout().green());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", style(e).for_stderr().red());
            return ExitCode::FAILURE;
        }
    }
}
