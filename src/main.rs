pub mod cmds;
pub mod cli;
pub mod error;
use std::process::ExitCode;
use console::style;
use clap::Parser;


fn main() -> ExitCode {
    match cli::Cli::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", style(e).for_stderr().red()); 
            return ExitCode::FAILURE;
        }
    }
}
