// Root CLI parser struct and entry point for argument parsing.

use clap::Parser;

use crate::cli::commands::Command;

#[derive(Parser)]
#[command(name = "corrode", version, about = "Codebase analysis engine")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short = 'L', long, help = "Print license information")]
    pub license: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
