pub mod cmds;
pub mod cli;
pub mod error;
use std::process::ExitCode;
use clap::Parser;


fn main() -> ExitCode {
    return cli::Cli::parse().run();
}
